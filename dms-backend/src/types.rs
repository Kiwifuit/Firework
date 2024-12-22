use std::sync::{Arc, RwLock};

use axum::{response::IntoResponse, Json};
use elytra::manifest::ElytraManifest;
use serde_json::json;
use tokio::sync::mpsc::Sender;
// use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default, Clone)]
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
  Build {
    params: ServerBuildParams,
    context: Arc<RwLock<crate::state::DMSState>>,
  },
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

#[derive(Debug, Serialize, Clone, Default)]
pub struct MinecraftServer {
  #[serde(skip)]
  pub manifest: Arc<ElytraManifest>,
  pub status: ServerStatus,
  pub players: PlayerStats,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct PlayerStats {
  online: u8,
  total: u8,
}

#[derive(Clone, Debug)]
pub struct Worker {
  pub id: usize,
  pub rx: Sender<WorkerMessage>, // main -> worker
  pub tx: Sender<WorkerMessage>, // worker -> main
}

pub enum DMSResponse<S, F> {
  Success(S),
  Fail(F),
}

impl<S, F> IntoResponse for DMSResponse<S, F>
where
  S: Serialize,
  F: Serialize,
{
  fn into_response(self) -> axum::response::Response {
    match self {
      Self::Success(data) => (
        axum::http::StatusCode::OK,
        Json(json!({
          "status": "success",
          "data": data
        })),
      ),
      Self::Fail(err) => (
        axum::http::StatusCode::BAD_REQUEST,
        Json(json!({
            "status": "error",
            "description": err
        })),
      ),
    }
    .into_response()
  }
}
