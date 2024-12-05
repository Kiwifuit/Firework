use log::debug;
use log::info;
use log::warn;
use thiserror::Error;

use crate::types::Worker;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use std::sync::mpsc;

#[derive(Debug, Error)]
pub enum StateError {
  #[error("I/O Error: {0}")]
  Io(#[from] std::io::Error),

  #[error("JSON de/serialization error: {0}")]
  Json(#[from] serde_json::Error),
}

#[derive(Clone)]
pub struct DMSState {
  // pub dispatcher: tokio::sync::broadcast::Sender<ServerBuildParams>,
  // pub available_servers: Arc<Mutex<HashMap<String, MinecraftServerManifest>>>,
  //   data_dir: PathBuf,
  workers: Vec<Worker>,
}

impl DMSState {
  pub fn new(worker_threads: usize) -> Result<Self, StateError> {
    let data_dir = PathBuf::from("./target/servers/");
    let mut workers = vec![];

    for task_id in 0..worker_threads {
      let (task_tx, task_rx) = mpsc::channel::<crate::types::WorkerJob>();
      let (server_tx, server_rx) = mpsc::channel::<crate::types::WorkerJob>();

      tokio::task::spawn(async move {
        info!("Spawned task #{}", task_id);

        while let Ok(msg) = task_rx.recv() {
          info!("Worker {} got message: {:?}", task_id, msg);

          server_tx.send(msg);
        }
      });

      workers.push(Worker {
        id: task_id,
        tx: task_tx,
        rx: Arc::new(server_rx),
      });
    }

    Ok(Self { workers })

    // Ok(Self {
    //   data_dir,
    //   available_servers: Arc::new(Mutex::new(manifests)),
    //   //   dispatcher,
    // })
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
