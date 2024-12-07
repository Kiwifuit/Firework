#[cfg(feature = "types")]
use serde::Deserialize;

use std::sync::Arc;

#[cfg(feature = "types")]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MavenArtifactVersions {
  pub group_id: Arc<str>,
  pub artifact_id: Arc<str>,
  pub versioning: MavenArtifactVersionVersioning,
}

#[cfg(feature = "types")]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MavenArtifactVersionVersioning {
  release: Arc<str>,
  latest: Arc<str>,
  last_updated: u64,
  versions: MAVersioningVersions,
}

#[cfg(feature = "types")]
impl MavenArtifactVersionVersioning {
  pub fn release(&self) -> &str {
    &self.release
  }

  pub fn latest(&self) -> &str {
    &self.latest
  }

  pub fn last_updated(&self) -> u64 {
    self.last_updated
  }

  pub fn versions(&self) -> Arc<[Arc<str>]> {
    self.versions.version.clone()
  }
}

#[cfg(feature = "types")]
#[derive(Debug, Deserialize)]
pub struct MAVersioningVersions {
  version: Arc<[Arc<str>]>,
}
