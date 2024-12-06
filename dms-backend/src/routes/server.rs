use std::sync::Arc;

use crate::{
  state::DMSState,
  types::{ServerBuildParams, WorkerMessage},
};
use axum::{extract::State, Json};
use log::info;

pub async fn get_servers(
  State(state): State<Arc<DMSState>>,
) -> Json<Vec<crate::types::MinecraftServer>> {
  info!("Querying servers");

  let resp = Vec::<crate::types::MinecraftServer>::new();

  resp.into()
}

pub async fn new_server(State(state): State<Arc<DMSState>>, Json(server): Json<ServerBuildParams>) {
  state.dispatch_job(WorkerMessage::Build(server)).await;
}
