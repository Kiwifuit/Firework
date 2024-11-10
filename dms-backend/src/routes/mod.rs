use crate::types::{MinecraftServer, ServerBuildParams};
use axum::Json;
use log::info;

pub async fn get_servers() -> Json<Vec<MinecraftServer>> {
  info!("Querying servers");
  let resp = Vec::new();

  info!("{} server(s) listed", resp.len());
  resp.into()
}

pub async fn new_server(Json(server): Json<ServerBuildParams>) {
  info!("Server build params: {:?}", server);
}
