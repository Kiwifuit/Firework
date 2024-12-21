use anyhow::{Context, Result};
use humantime::format_duration;
use log::{info, warn};
use temp_dir::TempDir;
use tokio::task::spawn;

use denji::{MinecraftServer, ServerSoftware};

use std::sync::mpsc::channel;
use std::time::Duration;

const CHANNEL_TIMEOUT: Duration = Duration::from_secs(90);

#[tokio::main]
async fn main() -> Result<()> {
  env_logger::init();

  let temp_dir = TempDir::with_prefix("denji_server")?;
  let server_installer = MinecraftServer::new(
    ServerSoftware::Forge,
    "1.20.4-49.1.4",
    "1.20.4",
    temp_dir.path().to_owned(),
  );
  let (tx, rx) = channel::<String>();
  let server_build = spawn(async move { server_installer.build_server(tx).await });

  info!(
    "started installer (timeout: {})",
    format_duration(CHANNEL_TIMEOUT)
  );

  loop {
    match rx.recv_timeout(CHANNEL_TIMEOUT) {
      Ok(line) => info!("{}", line),
      Err(e) => {
        warn!("{}. closing installer", e);
        break;
      }
    }
  }

  server_build
    .await
    .context("while trying to finish installer")?
    .context("while trying to install server")?;

  info!("you may test the channel and close this program when finished");

  Ok(())
}
