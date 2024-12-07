use anyhow::Context;
use futures_util::StreamExt;
use log::{debug, error, info};
use mar::types::MavenArtifact;
use mar::{get_artifact, get_versions};
use reqwest::get;
use std::ffi::OsStr;
use std::fmt::Display;
use std::fs::{create_dir, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use thiserror::Error;

mod post;
pub use post::agree_eula;

macro_rules! args {
    ($ ( $arg:expr ),+ $(,)?) => {
        vec![$($arg.as_ref(), )+]
    }
}

#[derive(Clone, Copy)]
pub enum ServerSoftware {
  Forge,
  Neoforge,
  Fabric,
  Quilt,
  Glowstone,
}

#[derive(Debug, Error)]
pub enum ServerInstallError {
  #[error("error while trying to fetch artifact data: {0}")]
  Artifact(#[from] mar::RepositoryError),
  #[error("version for artifact not found: {0}")]
  Version(String),
  #[error("net error: {0}")]
  Net(#[from] reqwest::Error),
  #[error("i/o error: {0}")]
  Io(#[from] std::io::Error),
  #[error("{0}")]
  Contextual(#[from] anyhow::Error),
}

pub struct MinecraftServer<S, I> {
  server: S,
  server_version: Arc<str>,
  game_version: Arc<str>,
  root_dir: I,
}

impl<I: AsRef<Path>, S: ServerSoftwareMeta> MinecraftServer<S, I> {
  pub fn new<V: ToString>(server: S, server_version: V, game_version: V, root_dir: I) -> Self {
    Self {
      server,
      server_version: server_version.to_string().into(),
      game_version: game_version.to_string().into(),
      root_dir,
    }
  }

  pub async fn build_server(&self, tx: Sender<String>) -> Result<(), ServerInstallError> {
    info!(
      "installing {} v{} for minecraft {} to {}",
      self.server,
      self.server_version,
      self.game_version,
      self.root_dir.as_ref().display()
    );

    self.download_server().await?;
    info!("installing server");

    let mut installer = Command::new("java");
    let installer = installer
      .args(vec![
        "-jar",
        self
          .root_dir
          .as_ref()
          .join("installer.jar")
          .to_str()
          .unwrap(),
      ])
      .args(
        self
          .server
          .installer_args(self.root_dir.as_ref(), &self.game_version),
      )
      .stdout(Stdio::piped());

    debug!("installing with args: {:?}", installer.get_args());

    if !self.root_dir.as_ref().exists() {
      error!(
        "{} does not exist! creating dir",
        self.root_dir.as_ref().display()
      );
      create_dir(&self.root_dir)?;
    }

    let mut installer = installer.spawn()?;
    {
      let stdout = BufReader::new(installer.stdout.as_mut().unwrap());

      for line in stdout.lines() {
        tx.send(line?)
          .context("while installing server (tx -> rx)")?;
      }
    }

    let stat = installer.wait()?;
    if !stat.success() {
      error!("installer exited with code {}", stat);
    }

    info!("installer exited with code {}", stat);
    info!("running post-install utilities");

    post::add_run_sh(&self.root_dir, self.server)?;
    post::write_user_jvm_args(&self.root_dir, "-Xms2G -Xmx8G -XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200 -XX:+UnlockExperimentalVMOptions -XX:+DisableExplicitGC -XX:+AlwaysPreTouch -XX:G1NewSizePercent=30 -XX:G1MaxNewSizePercent=40 -XX:G1HeapRegionSize=8M -XX:G1ReservePercent=20 -XX:G1HeapWastePercent=5 -XX:G1MixedGCCountTarget=4 -XX:InitiatingHeapOccupancyPercent=15 -XX:G1MixedGCLiveThresholdPercent=90 -XX:G1RSetUpdatingPauseTimePercent=5 -XX:SurvivorRatio=32 -XX:+PerfDisableSharedMem -XX:MaxTenuringThreshold=1 -Dusing.aikars.flags=https://mcflags.emc.gs -Daikars.new.flags=true")
            .context("while writing user_jvm_args.txt")?;
    Ok(())
  }

  async fn download_server(&self) -> Result<(), ServerInstallError> {
    let mut artifact: MavenArtifact = self.server.into();
    let versions = get_versions(&artifact).await?;

    if !versions
      .versioning
      .versions()
      .contains(&self.server_version.clone())
    {
      error!("unable to find version {}", self.server_version);

      return Err(ServerInstallError::Version(
        self.server.artifact_name(self.server_version.clone()),
      ));
    }

    info!("version {} resolved!", self.server_version);
    artifact.set_version(self.server_version.clone());

    let artifact_url = get_artifact(&artifact, self.server.artifact_name(&self.server_version))?;

    info!("resolved artifact to {:?}", artifact_url);
    info!("starting download...");
    let mut file = BufWriter::new(
      File::create_new(self.root_dir.as_ref().join("installer.jar"))
        .context("while creating file")?,
    );
    let mut stream = get(artifact_url).await?.bytes_stream();

    let mut total = 0;
    while let Some(chunk) = stream.next().await {
      total += file.write(&chunk?).context("while downloading file")?;
    }

    info!("finished download (downloaded {} bytes)", total);
    Ok(())
  }
}

pub trait ServerSoftwareMeta: Display + Into<MavenArtifact> + Copy {
  fn artifact_name<V: Display>(&self, version: V) -> String;
  fn installer_args<'a, I>(&self, installer_dir: &'a I, game_version: &'a str) -> Vec<&'a OsStr>
  where
    I: AsRef<OsStr> + ?Sized + 'a;
  fn run_sh_content(&self) -> Vec<String>;
}

impl Display for ServerSoftware {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Forge => "forge",
        Self::Neoforge => "neoforge",
        Self::Fabric => "fabric",
        Self::Quilt => "quilt",
        Self::Glowstone => "glowstone",
      }
    )
  }
}

impl From<ServerSoftware> for MavenArtifact {
  fn from(value: ServerSoftware) -> Self {
    match value {
      ServerSoftware::Forge => "maven.minecraftforge.net:net.minecraftforge:forge:",
      ServerSoftware::Neoforge => "maven.neoforged.net/releases:net.neoforged:neoforge:",
      ServerSoftware::Fabric => "maven.fabricmc.net:net.fabricmc:fabric-installer:",
      ServerSoftware::Quilt => "maven.quiltmc.org/repository/release:org.quiltm:quilt-installer:",
      ServerSoftware::Glowstone => {
        "repo.glowstone.net/content/repositories/snapshots:net.glowstone:glowstone:"
      }
    }
    .parse()
    .unwrap()
  }
}

impl ServerSoftwareMeta for ServerSoftware {
  fn artifact_name<V: Display>(&self, version: V) -> String {
    match self {
      Self::Forge => format!("forge-{}-installer.jar", version),
      Self::Neoforge => format!("neoforge-{}-installer.jar", version),
      Self::Quilt => format!("quilt-installer-{}.jar", version),
      Self::Fabric => format!("fabric-installer-{}.jar", version),
      Self::Glowstone => todo!(), // TODO: Fix this
    }
  }

  fn installer_args<'a, I>(&self, install_dir: &'a I, game_version: &'a str) -> Vec<&'a OsStr>
  where
    I: AsRef<OsStr> + ?Sized + 'a,
  {
    match self {
      Self::Forge => args!["--installServer", install_dir],
      Self::Neoforge => args!["--installServer", install_dir],
      Self::Quilt => args![
        "install",
        "server",
        game_version,
        "--install-dir",
        install_dir,
        "--create-scripts",
        "--download-server"
      ],
      Self::Fabric => args![
        "server",
        "-dir",
        install_dir,
        "-mcversion",
        game_version,
        "-downloadMinecraft",
      ],
      Self::Glowstone => todo!(), // TODO: Also this
    }
  }

  fn run_sh_content(&self) -> Vec<String> {
    match self {
      ServerSoftware::Fabric | ServerSoftware::Quilt => vec![
        "#!/usr/bin/env sh".to_string(),
        format!(
          "java -jar {}-server-launch.jar @user_jvm_args.txt \"$@\"",
          self.to_string()
        ),
      ],
      _ => vec![
        "#!/usr/bin/env sh".to_string(),
        "java -jar server.jar @user_jvm_args.txt \"$@\"".to_string(),
      ],
    }
  }
}
