use std::ffi::OsString;
// use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;

use anyhow::Context;
use futures_util::StreamExt;
use log::{debug, error, info};
use mar::{get_artifact, get_versions, types::MavenArtifact};
use reqwest::get;
use thiserror::Error;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::process::Command;

pub trait ServerParameters {
  /// The display name of the server
  fn name(&self) -> &str;

  /// The version of Minecraft to use
  fn version(&self) -> &str;

  /// The directory where the server will be built.
  ///
  /// This directory should be empty and should exist
  /// as `build_server` will not create this directory
  fn output_dir(&self) -> &Path;

  /// The args to the `run.sh` file.
  /// The shebang is already implied for this method
  fn run_args(&self) -> String;

  /// The args to be passed through the JVM.
  ///
  /// This method already provides the best arguments
  /// according to [this Reddit Post](https://www.reddit.com/r/feedthebeast/comments/5jhuk9/modded_mc_and_memory_usage_a_history_with_a/)
  /// on r/feedthebeast
  fn jvm_args(&self) -> &str {
    "-XX:+UseG1GC -Xmx4G -Xms4G -Dsun.rmi.dgc.server.gcInterval=2147483646 -XX:+UnlockExperimentalVMOptions -XX:G1NewSizePercent=20 -XX:G1ReservePercent=20 -XX:MaxGCPauseMillis=50 -XX:G1HeapRegionSize=32M"
  }

  /// Name of the installer / server software
  fn installer_name(&self) -> &str;

  /// The arguments to use when running the
  /// server installer
  fn installer_args(&self) -> Vec<OsString>;

  /// Name of the artifact to download
  fn artifact_name(&self) -> String;

  /// Artifact version. By default
  /// this returns the `.version()`
  fn artifact_version(&self) -> &str {
    self.version()
  }

  /// Artifact download directory.
  /// By default this returns `.output_dir()`
  fn artifact_dir(&self) -> &Path {
    self.output_dir()
  }
}

#[derive(Debug, Error)]
pub enum DenjiError {
  #[error("while downloading artifact: {0}")]
  Artifact(#[from] mar::RepositoryError),
  #[error("no artifact version was found")]
  NoVersion,
  #[error("I/O error: {0}")]
  Io(#[from] std::io::Error),
  #[error("http error:{0}")]
  Http(#[from] reqwest::Error),

  #[error("Installer returned code {0}")]
  Installer(i32),

  #[error("while writing run script: {0}")]
  WriteScript(anyhow::Error),

  #[error("while writing JVM args: {0}")]
  JvmArgs(anyhow::Error),
}

pub async fn build_server<P, F>(params: P, on_output_line: F) -> Result<(), DenjiError>
where
  P: ServerParameters + Into<MavenArtifact> + Clone,
  F: Fn(String),
{
  info!(
    "Downloading installer for {} v{} (for Minecraft {})",
    params.installer_name(),
    params.artifact_version(),
    params.version()
  );

  let jar_path = download_installer(
    params.clone(),
    &params.artifact_name(),
    &params.installer_name().to_lowercase(),
    params.artifact_dir(),
    params.artifact_version(),
  )
  .await?;

  let mut command = Command::new("java");
  let command = command
    .current_dir(params.output_dir())
    .arg("-jar")
    .arg(jar_path)
    .args(params.installer_args())
    .stdout(Stdio::piped())
    .kill_on_drop(true);

  let mut installer_command = command.spawn()?;
  {
    let stdout = BufReader::new(
      installer_command
        .stdout
        .as_mut()
        .expect("expected the installer's stdout to exist"),
    );
    let mut lines = stdout.lines();

    while let Some(line) = lines.next_line().await? {
      on_output_line(line);
    }
  }

  let stat = installer_command.wait().await?;

  #[expect(
    clippy::unwrap_used,
    reason = "`.unwrap` should be safe here, since we aren't killing the child proc"
  )]
  if !stat.success() {
    error!(
      "The server installer returned code: {}",
      stat.code().unwrap()
    );

    return Err(DenjiError::Installer(stat.code().unwrap()));
  }

  info!("Writing run script");
  write_run_script(&params)
    .await
    .map_err(DenjiError::WriteScript)?;

  info!("Writing JVM arguments");
  write_jvm_args(&params).await.map_err(DenjiError::JvmArgs)?;

  Ok(())
}

async fn write_run_script<P: ServerParameters>(params: &P) -> anyhow::Result<()> {
  #[cfg(unix)]
  let file_name = "run.sh";

  #[cfg(windows)]
  let file_name = "run.bat";

  // TODO: Buffered writers?
  let mut file = File::create(params.output_dir().join(file_name))
    .await
    .context(format!("while opening {} script", file_name))?;

  #[cfg(unix)]
  file.write_all(b"#!/usr/bin/env sh\n").await?;
  file.write_all(params.run_args().as_bytes()).await?;

  Ok(())
}

async fn write_jvm_args<P: ServerParameters>(params: &P) -> anyhow::Result<()> {
  let mut file = File::create(params.output_dir().join("user_jvm_args.txt"))
    .await
    .context("while opening JVM args file")?;

  file.write_all(params.jvm_args().as_bytes()).await?;

  Ok(())
}

async fn download_installer<A>(
  artifact: A,
  artifact_name: &str,
  installer: &str,
  out_dir: &Path,
  version: &str,
) -> Result<PathBuf, DenjiError>
where
  A: Into<MavenArtifact>,
{
  let mut artifact = artifact.into();

  resolve_version(&mut artifact, version).await?;
  info!("Downloading artifact");

  let artifact_url = get_artifact(&artifact, artifact_name)?;
  let artifact_path = out_dir.join(format!("installer-{}-{}.jar", installer, version));
  debug!("Downloading artifact to: {}", artifact_path.display());

  let mut file = BufWriter::new(File::create_new(&artifact_path).await?);
  let mut stream = get(artifact_url).await?.bytes_stream();

  let mut downloaded_bytes = 0;
  while let Some(chunk) = stream.next().await {
    downloaded_bytes += file.write(&chunk?).await?;
  }

  info!("Downloaded {} bytes", downloaded_bytes);

  Ok(artifact_path)
}

async fn resolve_version(artifact: &mut MavenArtifact, version: &str) -> Result<(), DenjiError> {
  let version: Arc<str> = Arc::from(version);
  let versions = get_versions(artifact).await?;
  let artifact_version = versions
    .versioning
    .versions()
    .iter()
    .find(|artifact_version| Arc::clone(artifact_version) == version.clone())
    .cloned();

  if artifact_version.is_none() {
    error!("Failed to resolve artifact version {:?}", version);
    return Err(DenjiError::NoVersion);
  }

  info!("Resolved artifact successfully!");
  let artifact_version = artifact_version.expect("expected artifact to be resolved at this point");
  artifact.set_version(artifact_version);

  Ok(())
}
