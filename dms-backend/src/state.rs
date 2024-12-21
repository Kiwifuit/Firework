use directories::ProjectDirs;
use log::debug;
use log::info;
use thiserror::Error;
use tokio::sync::mpsc::Receiver;

use crate::errors::StateError;
use crate::types::MinecraftServer;
use crate::types::Worker;
use crate::types::WorkerMessage;
use crate::worker::create_worker_thread;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::mpsc;

const WORKER_TIMEOUT: Duration = Duration::from_micros(20);

#[derive(Clone, Debug)]
pub struct DMSState {
  workers: Vec<Worker>,
  worker_channel: Arc<Receiver<WorkerMessage>>,
  pub data_dir: PathBuf,
  pub servers: Vec<MinecraftServer>,
}

impl DMSState {
  pub fn new(worker_threads: usize) -> Result<Self, StateError> {
    let project_dirs =
      ProjectDirs::from("tar.xz", "inhumane", "firework").ok_or(StateError::DataDirFail)?;

    let data_dir = project_dirs.data_dir().to_path_buf();

    debug!("Resolved data dir to: {}", data_dir.display());
    info!("Spinning {} worker thread(s)", worker_threads);

    let mut workers = vec![];
    let (server_tx, server_rx) = mpsc::channel::<crate::types::WorkerMessage>(100);

    for task_id in 0..worker_threads {
      let server_tx = server_tx.clone();

      workers.push(create_worker_thread(task_id, server_tx));
    }

    info!("Server is ready");

    Ok(Self {
      data_dir,
      workers,
      worker_channel: Arc::new(server_rx),
      servers: vec![],
    })
  }

  pub async fn dispatch_job(&self, job: WorkerMessage) -> Option<usize> {
    for worker in &self.workers {
      let job = job.clone();

      if worker.tx.send_timeout(job, WORKER_TIMEOUT).await.is_ok() {
        return Some(worker.id);
      }
      info!("Worker #{} is busy", worker.id);
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
