use anyhow::Context;
use log::info;
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

      handle_message(msg).context("while handling message")?;
    }

    Ok::<usize, anyhow::Error>(task_id)
  });

  Worker {
    id: task_id,
    tx: task_tx,
    rx: worker_tx,
  }
}

fn handle_message(message: WorkerMessage) -> anyhow::Result<()> {
  match message {
    WorkerMessage::Build(params) => build_server(params)?,
    _ => todo!(),
  }

  Ok(())
}

fn build_server(params: ServerBuildParams) -> anyhow::Result<()> {
  info!(
    "Building server {} for {} {}",
    params.name, params.server, params.server_version
  );

  Ok(())
}
