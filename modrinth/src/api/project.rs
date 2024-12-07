use super::{APIError, ENDPOINT};
use log::info;
use reqwest::Client;

use crate::types::project::ModrinthProject;
use crate::types::query::ProjectQuery;
use crate::types::result::SearchProjectHit;
use crate::types::result::SearchProjectResult;

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
///     let client = get_client().await.unwrap();
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
    .await
    .unwrap()
    .json()
    .await?;

  assert_eq!(resp.hits.len(), params.limit as usize);
  Ok(resp)
}

/// Gets a specific project, returned by `search_project`
/// ## Usage
/// ```
/// use modrinth::{get_project, search_project, get_client, ProjectQueryBuilder, IndexBy, Facet, Loader, ProjectType};
///
/// #[tokio::main]
/// async fn main() {
///     let client = get_client().await.unwrap();
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
      .await
      .unwrap()
      .json()
      .await?,
  )
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::api::get_client;
  use crate::types::query::ProjectQueryBuilder;
  use crate::types::Facet;
  use crate::types::Loader;
  use crate::types::{IndexBy, ProjectType};

  #[tokio::test]
  async fn check_search_projects() {
    let client = get_client().await.unwrap();

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
    let client = get_client().await.unwrap();

    let query = ProjectQueryBuilder::new()
      .query("kontraption")
      .limit(1)
      .index_by(IndexBy::Relevance)
      .build();

    let res = search_project(&client, &query).await.unwrap();

    let res = res.hits.first().unwrap();
    assert_eq!(res.project_id, "5yJ5IDKm".into()); // https://modrinth.com/mod/kontraption
    assert_eq!(res.project_type, ProjectType::Mod);

    let project = get_project(&client, res).await;

    assert!(project.is_ok());

    let project = project.unwrap();

    assert_eq!(project.id, "5yJ5IDKm".into());
    assert_eq!(project.project_type, ProjectType::Mod);
  }
}
