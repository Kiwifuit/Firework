use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct MinecraftServer {
  id: String,
  online: bool,
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
