use tokio::sync::mpsc::Sender;
// use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ServerStatus {
  Online,
  #[default]
  Offline,
  Building,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct ServerBuildParams {
  pub name: String,
  pub description: String,
  pub server: String,
  pub server_version: String,
}

#[derive(Debug, Clone)]
pub enum WorkerMessage {
  Build(ServerBuildParams),
  Start(MinecraftServer),
  Stop(MinecraftServer),
  Attach(MinecraftServer),

  Response(WorkerResponse),
}

#[derive(Debug, Clone)]
pub enum WorkerResponse {
  Received(usize),
}

// #[derive(Debug)]
// pub struct WorkerMessage {
//     from: String,
//     message: WorkerJob
// }

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MinecraftServer {
  manifest: ServerManifest,
}

#[derive(Clone)]
pub struct Worker {
  pub id: usize,
  pub rx: Sender<WorkerMessage>, // main -> worker
  pub tx: Sender<WorkerMessage>, // worker -> main
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerManifest {}

// #[derive(Debug, Serialize, Default)]
// pub struct MinecraftServer {
//   #[serde(flatten)]
//   manifest: MinecraftServerManifest,
//   software: String,
//   modpack: Option<String>,
//   status: ServerStatus,
//   players: MinecraftServerPlayers,
// }

// #[derive(Debug, Serialize, Default)]
// pub struct MinecraftServerPlayers {
//   online: u32,
//   total: u32,
// }

// #[derive(Debug, Serialize, Deserialize, Default)]
// pub struct MinecraftServerManifest {
//   pub id: String,
//   pub display_name: String,
//   pub description: String,
// }
