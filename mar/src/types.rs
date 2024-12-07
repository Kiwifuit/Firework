use std::str::FromStr;
use thiserror::Error;

mod deserialize;
pub use deserialize::*;

#[derive(Default)]
pub struct MavenArtifactBuilder<T> {
  pub(crate) base_url: Option<T>,
  pub(crate) group_id: Option<T>,
  pub(crate) artifact_id: Option<T>,
  pub(crate) version: Option<T>,
}

#[derive(Debug)]
pub struct MavenArtifact {
  pub(crate) base_url: String,
  pub(crate) group_id: String,
  pub(crate) artifact_id: String,
  pub(crate) version: Option<String>,
}

#[derive(Debug, Error)]
pub enum MavenArtifactParseError {
  #[error("input string is malformed: expected 3 semicolons, got {0}")]
  TooLittleSemiColons(usize),
  #[error("input string is malformed: expected at least 3 components")]
  NotEnoughComponents,
  #[error("input string is malformed")]
  Malformed,
}

impl FromStr for MavenArtifact {
  type Err = MavenArtifactParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.chars().filter(|c| *c == ':').count() {
      0 => return Err(MavenArtifactParseError::TooLittleSemiColons(0)),
      3 => (),
      other => return Err(MavenArtifactParseError::TooLittleSemiColons(other)),
    };

    let parts = s.split_terminator(':').collect::<Vec<&str>>();

    if parts.len() < 3 {
      return Err(MavenArtifactParseError::NotEnoughComponents);
    }

    let base_url = format!(
      "https://{}",
      parts.first().ok_or(MavenArtifactParseError::Malformed)?
    );
    let group_id = parts
      .get(1)
      .ok_or(MavenArtifactParseError::Malformed)?
      .to_string();
    let artifact_id = parts
      .get(2)
      .ok_or(MavenArtifactParseError::Malformed)?
      .to_string();
    let version = parts.get(3).map(|v| v.to_string());

    Ok(Self {
      base_url,
      group_id,
      artifact_id,
      version,
    })
  }
}

impl MavenArtifact {
  pub fn set_version<T: ToString>(&mut self, version: T) {
    self.version = Some(version.to_string());
  }
}

#[derive(Debug, Error)]
pub enum MavenArtifactBuildError {
  #[error("missing field: base_url. run with_base_url to fix")]
  BaseURL,
  #[error("missing field: group_id. run with_group_id to fix")]
  GroupID,
  #[error("missing field: artifact_id. run with_artifact_id to fix")]
  ArtifactID,
}

impl<T: ToString> MavenArtifactBuilder<T> {
  pub fn with_base_url(mut self, base_url: T) -> Self {
    self.base_url = Some(base_url);

    self
  }

  pub fn with_group_id(mut self, group_id: T) -> Self {
    self.group_id = Some(group_id);

    self
  }

  pub fn with_artifact_id(mut self, artifact_id: T) -> Self {
    self.artifact_id = Some(artifact_id);

    self
  }

  pub fn with_version(mut self, version: T) -> Self {
    self.version = Some(version);

    self
  }

  pub fn build(self) -> Result<MavenArtifact, MavenArtifactBuildError> {
    let base_url = self
      .base_url
      .ok_or(MavenArtifactBuildError::BaseURL)
      .map(|base_url| base_url.to_string())?;
    let group_id = self
      .group_id
      .ok_or(MavenArtifactBuildError::GroupID)
      .map(|group_id| group_id.to_string())?;
    let artifact_id = self
      .artifact_id
      .ok_or(MavenArtifactBuildError::ArtifactID)
      .map(|artifact_id| artifact_id.to_string())?;
    let version = self.version.map(|version| version.to_string());

    Ok(MavenArtifact {
      base_url,
      group_id,
      artifact_id,
      version,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_artifact_parsing() {
    // Assert that all of these are valid artifact strings
    assert!([
      "maven.minecraftforge.net:net.minecraftforge:forge:",
      "maven.neoforged.net/releases:net.neoforged:neoforge:",
      "maven.fabricmc.net:net.fabricmc:fabric-installer:",
      "maven.quiltmc.org/repository/release:org.quiltm:quilt-installer:",
      "repo.glowstone.net/content/repositories/snapshots:net.glowstone:glowstone:",
    ]
    .iter()
    .map(|artifact| artifact.parse::<MavenArtifact>())
    .all(|artifact: Result<_, _>| artifact.is_ok()));
  }
}
