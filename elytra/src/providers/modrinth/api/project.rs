use super::{APIError, ENDPOINT};
use log::info;
use reqwest::Client;

use crate::providers::modrinth::types::project::ModrinthProject;
use crate::providers::modrinth::types::query::ProjectQuery;
use crate::providers::modrinth::types::result::SearchProjectHit;
use crate::providers::modrinth::types::result::SearchProjectResult;

/// Searches Modrinth projects
///
/// Please refer to documentation on `ProjectQueryBuilder` for
/// information on this function's parameters
/// ## Usage
/// ```
/// use modrinth::{search_project, get_client, ProjectQueryBuilder, IndexBy, Facet, Loader};
///
/// #[tokio::main]
/// async fn main() {
///     let client = get_client().await.expect("Expected modrinth api client to be created");
///
///     let query = ProjectQueryBuilder::new()
///         .query("gravestones")
///         .limit(3)
///         .index_by(IndexBy::Relevance)
///         .facets(vec![
///             vec![Facet::Loader(Loader::Forge)],
///             vec![
///                 Facet::Category("adventure".to_string()),
///                 Facet::Category("utility".to_string()),
///             ],
///         ])
///         .build();
///
///     let res = search_project(&client, &query).await;
///
///     assert!(res.is_ok());
/// }
/// ```
pub async fn search_project(
  client: &Client,
  params: &ProjectQuery,
) -> Result<SearchProjectResult, APIError> {
  info!("Searching for project with params: {:?}", params);
  let resp: SearchProjectResult = client
    .get(format!("{}/v2/search", ENDPOINT))
    .query(params)
    .send()
    .await?
    // .text()
    // .await?;
    .json()
    .await?;

  //   dbg!(resp);

  //   panic!();

  assert!(
    resp.hits.len() <= params.limit as usize,
    "More hits were given than asked!"
  );
  Ok(resp)
}

/// Gets a specific project, returned by `search_project`
/// ## Usage
/// ```
/// use modrinth::{get_project, search_project, get_client, ProjectQueryBuilder, IndexBy, Facet, Loader, ProjectType};
///
/// #[tokio::main]
/// async fn main() {
///     let client = get_client().await.expect("Expected modrinth api client to be created");
///
///     let query = ProjectQueryBuilder::new()
///         .query("kontraption")
///         .limit(1)
///         .index_by(IndexBy::Relevance)
///         .build();
///
///     let res = search_project(&client, &query).await.unwrap();
///
///     let res = res.hits.first().unwrap();
///     assert_eq!(res.project_id, "5yJ5IDKm".into()); // https://modrinth.com/mod/kontraption
///     assert_eq!(res.project_type, ProjectType::Mod);
///
///     let project = get_project(&client, res).await;
///
///     assert!(project.is_ok());
///
///     let project = project.unwrap();
///
///     assert_eq!(project.id, "5yJ5IDKm".into());
///     assert_eq!(project.project_type, ProjectType::Mod);
/// }
/// ```
pub async fn get_project(
  client: &Client,
  project: &SearchProjectHit,
) -> Result<ModrinthProject, APIError> {
  info!("Getting project information for {}", project.title);
  Ok(
    client
      .get(format!("{}/v2/project/{}", ENDPOINT, project.project_id))
      .send()
      .await?
      .json()
      .await?,
  )
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::providers::modrinth::api::get_client;
  use crate::providers::modrinth::types::query::ProjectQueryBuilder;
  use crate::providers::modrinth::types::Facet;
  use crate::providers::modrinth::types::Loader;
  use crate::providers::modrinth::types::{IndexBy, ProjectType};

  #[tokio::test]
  async fn check_search_projects() {
    let client = get_client()
      .await
      .expect("expected Modrinth api to be reachable");

    let query = ProjectQueryBuilder::new()
      .query("gravestones")
      .limit(3)
      .index_by(IndexBy::Relevance)
      .facets(vec![
        vec![Facet::Loader(Loader::Forge)],
        vec![
          Facet::Category("adventure".to_string()),
          Facet::Category("utility".to_string()),
        ],
      ])
      .build();

    let res = search_project(&client, &query).await;

    assert!(res.is_ok());
  }

  #[tokio::test]
  async fn check_get_project() {
    let client = get_client()
      .await
      .expect("Expected modrinth api client to be created");

    let query = ProjectQueryBuilder::new()
      .query("kontraption")
      .limit(1)
      .index_by(IndexBy::Relevance)
      .build();

    let res = search_project(&client, &query)
      .await
      .expect("expected project query to succeed");

    let res = res
      .hits
      .first()
      .expect("expected at least 1 project hit, got none");
    assert_eq!(res.project_id, "5yJ5IDKm".into()); // https://modrinth.com/mod/kontraption
    assert_eq!(res.project_type, ProjectType::Mod);

    let project = get_project(&client, res).await;

    assert!(project.is_ok());

    let project = project.expect("expected modrinth project");

    assert_eq!(project.id, "5yJ5IDKm".into());
    assert_eq!(project.project_type, ProjectType::Mod);
  }
}
