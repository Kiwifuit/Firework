use std::rc::Rc;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ManifestError {
  #[error("I/O Error: {0}")]
  Io(#[from] std::io::Error),
  #[error("Error while deserializing manifest: {0}")]
  Deserialize(#[from] toml::de::Error),
  #[error("Error while serializing manifest: {0}")]
  Serialize(#[from] toml::ser::Error),
  #[cfg(feature = "providers-modrinth")]
  #[error("An error while performing a modrinth::api request: {0}")]
  ModrinthApi(#[from] APIError),
  #[error("Could not find a mod with slug {0:?}")]
  NoHits(String),
}

#[cfg(feature = "providers-modrinth")]
#[derive(Debug, Error)]
pub enum APIError {
  #[error("http error: {0}")]
  Http(#[from] reqwest::Error),

  #[error("dependency already resolved: {0}")]
  ResolvedDependency(Rc<str>),

  #[error("provided mod has no dependencies")]
  NoDependencies,

  #[error("provided mod has unresolvable dependencies")]
  UnresolvableDependency,
}
