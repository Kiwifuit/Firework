use std::fmt::{Debug, Display};

use super::{APIError, ENDPOINT};
use log::info;
use reqwest::Client;

use crate::providers::modrinth::types::query::VersionQuery;
use crate::providers::modrinth::types::version::ModrinthProjectVersion;
use crate::providers::modrinth::types::ModrinthProjectMeta;

#[expect(private_bounds)]
/// Lists versions of `project`
/// ## Usage
/// ```
/// use modrinth::{
///     get_versions, get_client, search_project, IndexBy, Loader, ProjectQueryBuilder,
///     VersionQueryBuilder,
/// };
///
/// #[tokio::main]
/// async fn main() {
///     let client = get_client().await
///         .expect("expected Modrinth api to be reachable");
///
///     let query = ProjectQueryBuilder::new()
///         .query("kontraption")
///         .limit(1)
///         .index_by(IndexBy::Relevance)
///         .build();
///
///     let res = search_project(&client, &query).await.unwrap();
///     let project = res.hits.first().unwrap();
///
///     let v_query = VersionQueryBuilder::new()
///         .featured(true)
///         .versions(vec!["1.20.1"])
///         .loaders(vec![Loader::Forge])
///         .build();
///
///     let version = get_versions(&client, &project, &v_query).await;
///
///     assert!(version.is_ok());
///     assert!(!version.unwrap().is_empty());
/// }
/// ```
pub async fn get_versions<M>(
  client: &Client,
  project: &M,
  params: &VersionQuery,
) -> Result<Vec<ModrinthProjectVersion>, APIError>
where
  M: ModrinthProjectMeta,
  <M as ModrinthProjectMeta>::Id: Display,
{
  info!("Searching for versions with params: {:?}", params);

  //   :Vec<ModrinthProjectVersion>
  let resp = client
    // TODO: ADD ERROR
    .get(format!(
      "{}/v2/project/{}/version",
      ENDPOINT,
      project
        .project_id()
        .expect("expected project id or slug to exist")
    ))
    .query(params)
    .send()
    .await?
    // .text()
    .json::<Vec<ModrinthProjectVersion>>()
    .await?;

  Ok(resp)
}

pub(crate) async fn get_version<M>(
  client: &Client,
  project: &M,
) -> Result<ModrinthProjectVersion, APIError>
where
  M: ModrinthProjectMeta,
  <M as ModrinthProjectMeta>::Id: Display + Debug,
{
  info!(
    "Searching for version: {:?}",
    project.version_id().expect("expected version id to exit")
  );

  let resp: ModrinthProjectVersion = client
    // TODO: ADD ERROR
    .get(format!(
      "{}/v2/version/{}",
      ENDPOINT,
      project.version_id().expect("expected version id to exit")
    ))
    .send()
    .await?
    .json()
    .await?;

  Ok(resp)
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::providers::modrinth::{
    get_client, search_project, IndexBy, Loader, ProjectQueryBuilder, VersionQueryBuilder,
  };

  #[tokio::test]
  async fn check_get_versions() {
    let client = get_client()
      .await
      .expect("expected Modrinth api to be reachable");

    let query = ProjectQueryBuilder::new()
      .query("kontraption")
      .limit(1)
      .index_by(IndexBy::Relevance)
      .build();

    let res = search_project(&client, &query)
      .await
      .expect("expected project query to succeed");

    let project = res
      .hits
      .first()
      .expect("expected at least 1 project hit, got none");

    let v_query = VersionQueryBuilder::new()
      .featured(true)
      .versions(vec!["1.20.1"])
      .loaders(vec![Loader::Forge])
      .build();

    let version = get_versions(&client, &project, &v_query).await;

    assert!(version.is_ok());
    assert!(!version
      .expect("expected version list to be not empty")
      .is_empty());
  }
}
