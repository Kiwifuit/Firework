use crate::types::*;

use log::debug;
use quick_xml::de::from_str;
use reqwest::get;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
  #[error("version already available")]
  VersionAvailable,

  #[error("version not available")]
  NoVersion,

  #[error("http error: {0}")]
  Http(#[from] reqwest::Error),

  #[error("xml deserialization error: {0}")]
  XmlParse(#[from] quick_xml::DeError),
}

impl Serialize for RepositoryError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(&self.to_string())
  }
}

pub async fn get_versions(
  artifact: &MavenArtifact,
) -> Result<MavenArtifactVersions, RepositoryError> {
  if artifact.version.is_some() {
    Err(RepositoryError::VersionAvailable)
  } else {
    debug!(
      "got url: {}",
      format!(
        "{}/{}/{}/maven-metadata.xml",
        artifact.base_url,
        artifact.group_id.replace('.', "/"),
        artifact.artifact_id
      )
    );

    let raw = get(format!(
      "{}/{}/{}/maven-metadata.xml",
      artifact.base_url,
      artifact.group_id.replace('.', "/"),
      artifact.artifact_id
    ))
    .await?
    .text()
    .await?;

    let parsed = from_str(&raw)?;

    Ok(parsed)
  }
}

pub fn get_artifact<T: ToString>(
  artifact_data: &MavenArtifact,
  artifact_name: T,
) -> Result<String, RepositoryError> {
  if artifact_data.version.is_none() {
    Err(RepositoryError::NoVersion)
  } else {
    let artifact_url = format!(
      "{}/{}/{}/{}/{}",
      artifact_data.base_url,
      artifact_data.group_id.replace('.', "/"),
      artifact_data.artifact_id,
      artifact_data
        .version
        .clone()
        .expect("expected version to be present"),
      artifact_name.to_string()
    );

    debug!("artifact url: {}", artifact_url);
    Ok(artifact_url)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn versions() {
    let artifact = MavenArtifactBuilder::default()
      .with_base_url("https://maven.minecraftforge.net")
      .with_artifact_id("forge")
      .with_group_id("net.minecraftforge")
      .build()
      .expect("expected the artifact to be built");

    let versions = get_versions(&artifact).await;

    assert!(versions.is_ok());
  }

  #[tokio::test]
  async fn version() {
    let mut artifact = MavenArtifactBuilder::default()
      .with_base_url("https://maven.minecraftforge.net")
      .with_artifact_id("forge")
      .with_group_id("net.minecraftforge")
      .build()
      .expect("expected the artifact to be built");

    let versions_list = get_versions(&artifact)
      .await
      .expect("expected `get_versions` to work");
    let selected_version = versions_list.versioning.latest();

    artifact.set_version(selected_version);
    let artifact_name = format!("forge-{}-installer.jar", selected_version);
    let expected_artifact_url = format!(
      "https://maven.minecraftforge.net/net/minecraftforge/forge/{0}/forge-{0}-installer.jar",
      selected_version
    );
    let artifact_url = get_artifact(&artifact, artifact_name);

    assert!(artifact_url.is_ok());
    let artifact_url = artifact_url.expect("Expected url to exist");
    assert_eq!(artifact_url, expected_artifact_url);
  }
}
