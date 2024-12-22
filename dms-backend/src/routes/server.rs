use std::{
  fs::{create_dir, read_dir, OpenOptions},
  io::BufReader,
  path::{Path, PathBuf},
  sync::{Arc, RwLock},
};

use crate::{
  errors::ServerError,
  state::DMSState,
  types::{DMSResponse, MinecraftServer, ServerBuildParams, WorkerMessage},
};
use axum::{extract::State, Json};
use log::{debug, error, info, warn};

#[axum::debug_handler]
pub async fn get_servers(
  State(state): State<Arc<RwLock<DMSState>>>,
) -> DMSResponse<Vec<crate::types::MinecraftServer>, ServerError> {
  {
    debug!("Listing servers...");
    let mut state = state
      .write()
      .expect("lock was poisoned before the servers could be listed");

    state.list_servers();
    debug!("{} servers listed", state.servers.len());
  }

  DMSResponse::Success(
    state
      .read()
      .expect("lock was poisoned after servers were listed")
      .servers
      .clone(),
  )
}

pub async fn new_server(
  State(state): State<Arc<RwLock<DMSState>>>,
  Json(server): Json<ServerBuildParams>,
) -> DMSResponse<String, String> {
  let dispatcher = {
    let state = state
      .read()
      .expect("lock was poisoned before build job was dispatched");

    state.clone()
  };

  match dispatcher
    .dispatch_job(WorkerMessage::Build {
      context: state.clone(),
      params: server,
    })
    .await
  {
    Some(wid) => DMSResponse::Success(format!("Dispatched job to worker #{wid}")),
    None => DMSResponse::Fail("No workers are available at this moment".to_string()),
  }
}
