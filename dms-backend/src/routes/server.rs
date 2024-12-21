use std::{fs::read_dir, sync::Arc};

use crate::{
  errors::ServerError,
  state::DMSState,
  types::{ServerBuildParams, WorkerMessage},
};
use axum::{extract::State, Json};
use log::{info, warn};

pub async fn get_servers(
  State(state): State<Arc<DMSState>>,
) -> Result<Json<Vec<crate::types::MinecraftServer>>, ServerError> {
  info!("Querying servers");

  let servers = read_dir(&state.data_dir)?
    .map_while(|entry| match entry {
      Err(err) => {
        warn!("Error while fetching direntry: {}", err);
        None
      }
      Ok(ent) => {
        let path = ent.path();

        info!("Found server: {}", path.display());
        Some(path)
      }
    })
    .collect::<Vec<_>>();

  info!("Found {} server(s) total", servers.len());

  Ok(vec![].into())
}

pub async fn new_server(State(state): State<Arc<DMSState>>, Json(server): Json<ServerBuildParams>) {
  state
    .dispatch_job(WorkerMessage::Build {
      context: state.clone(),
      params: server,
    })
    .await;
}
