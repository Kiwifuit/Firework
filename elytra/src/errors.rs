use thiserror::Error;

#[derive(Debug, Error)]
pub enum ManifestError {
  #[error("I/O Error: {0}")]
  Io(#[from] std::io::Error),
  #[error("Error while deserializing manifest: {0}")]
  Deserialize(#[from] toml::de::Error),
  #[error("Error while serializing manifest: {0}")]
  Serialize(#[from] toml::ser::Error),
  #[error("An error while performing a modrinth::api request: {0}")]
  ModrinthApi(#[from] modrinth::APIError),
  #[error("Could not find a mod with slug {0:?}")]
  NoHits(String),
}
