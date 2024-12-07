use crate::types::Loader;
use serde::Serialize;

#[derive(Debug, Serialize)]
/// Represents a built complex search query for
/// `get_versions`.
pub struct VersionQuery {
  #[serde(
    skip_serializing_if = "Vec::is_empty",
    serialize_with = "crate::types::serialize_vec_urlencoded"
  )]
  pub(crate) loaders: Vec<Loader>,
  #[serde(
    skip_serializing_if = "Vec::is_empty",
    serialize_with = "crate::types::serialize_vec_urlencoded"
  )]
  pub(crate) game_versions: Vec<String>,
  pub(crate) featured: bool,
}

#[derive(Debug, Default)]
/// Represents a complex search query for
/// `get_versions`. Use `.build()` to build
/// the query
pub struct VersionQueryBuilder {
  pub loaders: Option<Vec<Loader>>,
  pub versions: Option<Vec<String>>,
  pub featured: Option<bool>,
}

impl VersionQueryBuilder {
  /// Creates a new query
  pub fn new() -> Self {
    Self::default()
  }

  /// Version must support being loaded by...
  pub fn loaders(mut self, loaders: Vec<Loader>) -> Self {
    self.loaders = Some(loaders);
    self
  }

  /// Version must support Minecraft version...
  pub fn versions<A: ToString>(mut self, versions: Vec<A>) -> Self {
    self.versions = Some(versions.iter().map(|a| a.to_string()).collect());
    self
  }

  /// Version has to be featured
  pub fn featured(mut self, featured: bool) -> Self {
    self.featured = Some(featured);
    self
  }

  /// Build the query
  pub fn build(self) -> VersionQuery {
    VersionQuery {
      loaders: self.loaders.unwrap_or_default(),
      game_versions: self.versions.unwrap_or_default(),
      featured: self.featured.unwrap_or_default(),
    }
  }
}
