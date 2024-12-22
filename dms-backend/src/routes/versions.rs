use std::str::FromStr;

use axum::response::IntoResponse;
use axum::{extract::Path, Json};
use http::StatusCode;
use log::info;
use mar::get_versions;
use mar::types::MavenArtifact;
use owo_colors::OwoColorize;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

use crate::types::DMSResponse;

#[derive(Debug, Serialize, Error)]
pub enum VersionError {
  #[error("Failed to parse artifact id: {reason}")]
  Artifact { reason: String },
  #[error("Repository error: {0}")]
  Repository(#[from] mar::RepositoryError),
}

impl IntoResponse for VersionError {
  fn into_response(self) -> axum::response::Response {
    let status = match &self {
      Self::Artifact { .. } => StatusCode::BAD_REQUEST,
      Self::Repository(..) => StatusCode::INTERNAL_SERVER_ERROR,
    };

    let body = Json(json!({ "error": self }));

    (status, body).into_response()
  }
}

pub async fn get_loaders() -> DMSResponse<Vec<String>, ()> {
  DMSResponse::Success(
    [
      "NeoForge",
      "Forge",
      "Fabric",
      "Quilt",
      "Glowstone",
      "Arclight",
    ]
    .iter()
    .map(ToString::to_string)
    .collect::<Vec<_>>(),
  )
}

pub async fn get_loader_version(
  Path(loader): Path<String>,
) -> DMSResponse<Vec<String>, VersionError> {
  info!("Querying versions for loader: {}", loader.blue());

  // TODO: Revamp this piece of shit and
  //       use FromResidual & Try to make a
  //       `Result<T, E>` custom impl once
  //       https://github.com/rust-lang/rust/issues/84277
  //       has been stabilized, hopefully
  let artifact =
    match get_loader_artifact(loader.to_lowercase().as_str()).ok_or(VersionError::Artifact {
      reason: format!("No such loader: {}", loader),
    }) {
      Ok(a) => a,
      Err(e) => return DMSResponse::Fail(e),
    };

  let artifact_versions = match get_versions(&artifact).await {
    Ok(versions) => versions,
    Err(err) => {
      return DMSResponse::Fail(VersionError::Repository(err));
    }
  };

  let versions = artifact_versions
    .versioning
    .versions()
    .iter()
    .map(ToString::to_string)
    .collect::<Vec<_>>();

  info!("Got {} version(s) for loader", versions.len().yellow());

  DMSResponse::Success(versions)
}

fn get_loader_artifact(loader_type: &str) -> Option<MavenArtifact> {
  match loader_type {
    "forge" => Some("maven.minecraftforge.net:net.minecraftforge:forge:"),
    "fabric" => Some("maven.fabricmc.net:net.fabricmc:fabric-installer:"),
    "quilt" => Some("maven.quiltmc.org/repository/release:org.quiltmc:quilt-installer:"),
    "neoforge" => Some("maven.neoforged.net/releases:net.neoforged:neoforge:"),
    "glowstone" => {
      Some("repo.glowstone.net/content/repositories/snapshots:net.glowstone:glowstone:")
    }
    "arclight" => Some("maven.izzel.io/releases/:io.izzel.arclight:arclight-forge:"),
    _ => None,
  }
  .and_then(|a| MavenArtifact::from_str(a).ok())
}
