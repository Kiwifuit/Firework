use std::{
  fs::{create_dir, read_dir, OpenOptions},
  io::BufReader,
  path::{Path, PathBuf},
  sync::Arc,
};

use crate::{
  errors::ServerError,
  state::DMSState,
  types::{MinecraftServer, ServerBuildParams, WorkerMessage},
};
use axum::{extract::State, Json};
use log::{debug, error, info, warn};
use owo_colors::OwoColorize;

pub async fn get_servers(
  State(state): State<Arc<DMSState>>,
) -> Result<Json<Vec<crate::types::MinecraftServer>>, ServerError> {
  info!("Querying servers");

  debug!(
    "Searching for servers within: {}",
    state.data_dir.display().magenta()
  );
  let servers = if !state.data_dir.exists() {
    warn!("Creating data dir");
    create_dir(&state.data_dir)?;

    vec![]
  } else {
    read_dir(&state.data_dir)?
      .map_while(|entry| match entry {
        Err(err) => {
          warn!("Error while fetching direntry: {}", err);
          None
        }
        Ok(ent) => {
          let path = ent.path();

          info!("Found server: {}", path.display());

          read_server(&path)
            .map_err(|e| {
              error!("An error occured while reading server data: {}", e);
              warn!(
                "The server on {} will not be listed",
                path.display().magenta()
              )
            })
            .ok()
        }
      })
      .collect::<Vec<_>>()
  };

  info!("Found {} server(s) total", servers.len());

  Ok(servers)
}

pub async fn new_server(State(state): State<Arc<DMSState>>, Json(server): Json<ServerBuildParams>) {
  state
    .dispatch_job(WorkerMessage::Build {
      context: state.clone(),
      params: server,
    })
    .await;
}

fn read_server(path: &Path) -> Result<MinecraftServer, ServerError> {
  let mut manifest_file = BufReader::new(
    OpenOptions::new()
      .read(true)
      .write(false)
      .open(path.join("server.json"))?,
  );

  let manifest = serde_json::from_reader(manifest_file)?;

  Ok(MinecraftServer {
    manifest,
    ..Default::default()
  })
}
