use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ServerStatus {
  Online,
  #[default]
  Offline,
  Building,
}

#[derive(Debug, Serialize, Default)]
pub struct MinecraftServer {
  id: String,
  status: ServerStatus,
  display_name: String,
  description: String,
  players: MinecraftServerPlayers,
  software: String,
  modpack: Option<String>,
}

#[derive(Debug, Serialize, Default)]
pub struct MinecraftServerPlayers {
  online: u32,
  total: u32,
}

#[derive(Debug, Deserialize, Default)]
pub struct ServerBuildParams {
  name: String,
  description: String,
  server: String,
  server_version: String,
}
