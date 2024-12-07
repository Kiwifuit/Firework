use anyhow::Context;
use log::{error, info, warn};
use tokio::sync::mpsc::{channel, Sender};

use crate::types::{ServerBuildParams, Worker, WorkerMessage, WorkerResponse};

pub fn create_worker_thread(task_id: usize, server_tx: Sender<WorkerMessage>) -> Worker {
  let (task_tx, mut task_rx) = channel::<crate::types::WorkerMessage>(100);
  let worker_tx = server_tx.clone();

  tokio::task::spawn(async move {
    info!("Spawned worker #{}", task_id);

    while let Some(msg) = task_rx.recv().await {
      info!("Worker {} got message: {:?}", task_id, msg);

      server_tx
        .send(WorkerMessage::Response(WorkerResponse::Received(task_id)))
        .await
        .context("while sending confirmation message")?;

      handle_message(msg)
        .await
        .context("while handling message")?;
    }

    Ok::<usize, anyhow::Error>(task_id)
  });

  Worker {
    id: task_id,
    tx: task_tx,
    rx: worker_tx,
  }
}

async fn handle_message(message: WorkerMessage) -> anyhow::Result<()> {
  match message {
    WorkerMessage::Build(params) => build_server(params).await?,
    _ => todo!(),
  }

  Ok(())
}

async fn build_server(params: ServerBuildParams) -> anyhow::Result<()> {
  info!(
    "Building server {} for {} {}",
    params.name, params.server, params.server_version
  );
  let (tx, rx) = std::sync::mpsc::channel::<String>();

  info!("Spawning read thread");
  std::thread::spawn(move || loop {
    match rx.recv_timeout(std::time::Duration::from_secs(90)) {
      Ok(line) => info!("{}", line),
      Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
        error!("Timeout reached while awaiting message");
      }
      Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
    }
  });

  let server_build = denji::MinecraftServer::new(
    params.server.parse::<denji::ServerSoftware>().unwrap(),
    &params.server_version,
    &params.server_version,
    "target/servers/dummy/",
  );
  let task = tokio::task::spawn(async move {
    if let Err(e) = server_build.build_server(tx).await {
      error!("An error occurred while building the server: {:?}", e);
    }
  })
  .await
  .expect("expected this task to not return any errors");

  Ok(())
}
