use std::rc::Rc;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ManifestError {
  #[error("I/O Error: {0}")]
  Io(#[from] std::io::Error),
  #[error("An error while performing a modrinth::api request: {0}")]
  #[cfg(feature = "providers-modrinth")]
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
