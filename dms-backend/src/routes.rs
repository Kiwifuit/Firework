use crate::types::MinecraftServer;
use axum::Json;
use log::info;

pub async fn get_servers() -> Json<Vec<MinecraftServer>> {
  info!("Querying servers");
  let resp = (0..3)
    .map(|_| MinecraftServer::default())
    .collect::<Vec<_>>();

  info!("{} server(s) listed", resp.len());
  resp.into()
}
