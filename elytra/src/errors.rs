use thiserror::Error;

#[derive(Debug, Error)]
pub enum ManifestError {
  #[error("I/O Error: {0}")]
  Io(#[from] std::io::Error),
  #[error("Error while deserializing manifest: {0}")]
  Deserialize(#[from] toml::de::Error),
  #[error("Error while serializing manifest: {0}")]
  Serialize(#[from] toml::ser::Error),
}
