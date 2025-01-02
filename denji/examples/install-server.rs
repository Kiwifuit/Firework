use anyhow::{Context, Result};
use log::{debug, info, warn};
use mar::types::MavenArtifact;

use denji::args;
use denji::shell::{build_server, ServerParameters};

use std::env::var;
use std::fs::create_dir;
use std::path::PathBuf;
use std::sync::mpsc::channel;
// use std::sync::Arc;
use std::thread::spawn;
// use std::time::Duration;

// const CHANNEL_TIMEOUT: Duration = Duration::from_secs(90);

#[derive(Clone)]
struct ForgeServer {
  pub name: String,
  pub build_dir: PathBuf,
  pub artifact_dir: PathBuf,
}

impl ForgeServer {
  fn new(name: String) -> anyhow::Result<Self> {
    #[cfg(unix)]
    let default_tmpdir = "/tmp";
    #[cfg(windows)]
    let default_tmpdir = ".";

    let tmpdir = var("TMP").unwrap_or(default_tmpdir.to_string());

    let build_dir = PathBuf::from(&tmpdir).join("denji-build");
    let artifact_dir = PathBuf::from(tmpdir).join("denji-artifact");

    debug!("{:?} will be installed to {}", name, build_dir.display());
    debug!(
      "Artifacts will be downloaded to: {}",
      artifact_dir.display()
    );

    if let Err(e) = create_dir(&build_dir) {
      warn!("An error occurred while creating the build directory: {e}")
    }
    if let Err(e) = create_dir(&artifact_dir) {
      warn!("An error occurred while creating the artifact directory: {e}")
    }

    Ok(Self {
      name,
      build_dir,
      artifact_dir,
    })
  }
}

impl ServerParameters for ForgeServer {
  fn name(&self) -> &str {
    &self.name
  }

  fn version(&self) -> &str {
    "1.20.1"
  }

  fn output_dir(&self) -> &std::path::Path {
    &self.build_dir
  }

  fn artifact_dir(&self) -> &std::path::Path {
    &self.artifact_dir
  }

  fn installer_name(&self) -> &str {
    "Forge"
  }

  fn artifact_name(&self) -> String {
    format!("forge-{}-installer.jar", self.artifact_version())
  }

  fn artifact_version(&self) -> &str {
    "1.20.1-47.3.22"
  }

  fn installer_args(&self) -> Vec<std::ffi::OsString> {
    args!["--installServer", &self.build_dir]
  }

  fn run_args(&self) -> String {
    format!("java -jar libraries/net/minecraftforge/forge/{0}/forge-{0}-server.jar @usr_jvm_args.txt nogui \"$@\"", self.artifact_version())
  }
}

impl From<ForgeServer> for MavenArtifact {
  fn from(_value: ForgeServer) -> Self {
    "maven.minecraftforge.net:net.minecraftforge:forge:"
      .parse()
      .expect("expected `mar` to parse this string")
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  env_logger::init();
  info!("Initializing Forge server builder");
  let server =
    ForgeServer::new("Hello world!".to_string()).context("while constructing server metadata")?;

  let (tx_logs, rx_logs) = channel();
  let read_thread = spawn(move || {
    while let Ok(line) = rx_logs.recv() {
      info!("{}", line)
    }
  });

  build_server(server, move |line| {
    tx_logs
      .send(line)
      .expect("expected to send line to receiver half");
  })
  .await
  .context("while building server")?;

  #[expect(clippy::unwrap_used, reason = "This thread cannot panic")]
  read_thread.join().unwrap();

  Ok(())
}
