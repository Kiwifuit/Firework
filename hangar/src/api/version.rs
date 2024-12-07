use std::fmt::Display;

use crate::types::{
  project::HangarProject, query::version::VersionQuery, version::HangarVersion, HangarPlatform,
  HangarVersions,
};
use reqwest::Client;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VersionError {
  #[error("http error: {0}")]
  Http(#[from] reqwest::Error),
}

pub async fn get_versions(
  client: &Client,
  project: &HangarProject,
  params: &VersionQuery,
) -> Result<HangarVersions, VersionError> {
  Ok(
    client
      .get(format!(
        "{}/api/v1/projects/{}/versions",
        super::HANGAR_ENDPOINT,
        project.namespace.slug
      ))
      .query(params)
      .send()
      .await?
      .json()
      .await?,
  )
}

pub async fn get_version(
  client: &Client,
  project: &HangarProject,
  version: String,
  params: &VersionQuery,
) -> Result<HangarVersion, VersionError> {
  Ok(
    client
      .get(format!(
        "{}/api/v1/projects/{}/versions/{}",
        super::HANGAR_ENDPOINT,
        project.namespace.slug,
        version
      ))
      .query(params)
      .send()
      .await?
      .json()
      .await?,
  )
}

pub fn get_download_link<T: Display>(slug: T, name: T, platform: HangarPlatform) -> String {
  format!(
    "{}/api/v1/projects/{}/versions/{}/{}/download",
    super::HANGAR_ENDPOINT,
    slug,
    name,
    platform
  )
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{search_project, SearchQueryBuilder, VersionQueryBuilder};

  #[tokio::test]
  async fn test_get_versions() {
    let client = Client::new();
    let pquery = SearchQueryBuilder::default()
      .query("ViaVersion")
      .version("1.20.1")
      .build();

    let projects = search_project(&client, &pquery).await.unwrap();
    let project = projects.result.first().unwrap();

    let vquery = VersionQueryBuilder::default()
      .platform(HangarPlatform::Paper)
      .build();
    let versions = get_versions(&client, project, &vquery).await;

    assert!(versions.is_ok_and(|v| !v.result.is_empty()))
  }
}
