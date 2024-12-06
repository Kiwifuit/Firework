use thiserror::Error;
use tokio::sync::mpsc::Receiver;

use crate::types::Worker;
use crate::types::WorkerMessage;
use crate::types::WorkerResponse;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::mpsc;

const WORKER_TIMEOUT: Duration = Duration::from_micros(20);

#[derive(Debug, Error)]
pub enum StateError {
  #[error("I/O Error: {0}")]
  Io(#[from] std::io::Error),

  #[error("JSON de/serialization error: {0}")]
  Json(#[from] serde_json::Error),
}

#[derive(Clone)]
pub struct DMSState {
  workers: Vec<Worker>,
  worker_channel: Arc<Receiver<WorkerMessage>>,
  data_dir: PathBuf,
}

impl DMSState {
  pub fn new(worker_threads: usize) -> Result<Self, StateError> {
    let data_dir = PathBuf::from("./target/servers/");
    let mut workers = vec![];
    let (server_tx, server_rx) = mpsc::channel::<crate::types::WorkerMessage>(100);

    for task_id in 0..worker_threads {
      let (task_tx, mut task_rx) = mpsc::channel::<crate::types::WorkerMessage>(100);
      let server_tx = server_tx.clone();
      let worker_tx = server_tx.clone();

      tokio::task::spawn(async move {
        info!("Spawned worker #{}", task_id);

        while let Some(msg) = task_rx.recv().await {
          info!("Worker {} got message: {:?}", task_id, msg);
          //   server_tx.send(msg).await;
          server_tx
            .send(WorkerMessage::Response(WorkerResponse::Received(task_id)))
            .await;
        }
      });

      workers.push(Worker {
        id: task_id,
        tx: task_tx,
        rx: worker_tx,
      });
    }

    Ok(Self {
      data_dir,
      workers,
      worker_channel: Arc::new(server_rx),
    })
  }

  pub async fn dispatch_job(&self, job: WorkerMessage) -> Option<usize> {
    for worker in &self.workers {
      let job = job.clone();

      if worker.tx.send_timeout(job, WORKER_TIMEOUT).await.is_ok() {
        return Some(worker.id);
      }
    }

    None
  }
}

// fn fetch_manifest<P: AsRef<Path>>(path: P) -> Result<MinecraftServerManifest, StateError> {
//   let manifest_path = path.as_ref().join("manifest.json");
//   let manifest_file = OpenOptions::new().read(true).open(manifest_path)?;
//   let manifest_reader = BufReader::new(manifest_file);

//   Ok(serde_json::from_reader(manifest_reader)?)
// }

// fn get_dirs_from_entries(entry: Result<std::fs::DirEntry, std::io::Error>) -> Option<PathBuf> {
//   let dir_entry = if let Err(err) = entry {
//     warn!("Skipping entry because of error: {}", err);
//     None
//   } else {
//     entry.ok()
//   }?;

//   if dir_entry.metadata().is_ok_and(|m| m.is_file()) {
//     None
//   } else {
//     let path = dir_entry.path();
//     debug!("Assuming that {} is a directory", path.display());
//     Some(path)
//   }
// }
