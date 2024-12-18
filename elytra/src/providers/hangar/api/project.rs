use crate::types::{query::search::SearchQuery, HangarProjects};
use reqwest::Client;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectError {
  #[error("http error: {0}")]
  Http(#[from] reqwest::Error),
}

pub async fn search_project(
  client: &Client,
  params: &SearchQuery,
) -> Result<HangarProjects, ProjectError> {
  Ok(
    client
      .get(format!("{}/api/v1/projects", super::HANGAR_ENDPOINT))
      .query(params)
      .send()
      .await?
      .json()
      .await?,
  )
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::SearchQueryBuilder;

  #[tokio::test]
  async fn test_search_project() {
    let client = Client::new();
    let query = SearchQueryBuilder::default()
      .query("ViaVersion")
      .version("1.20.1")
      .build();

    let projects = search_project(&client, &query).await;
    assert!(projects.is_ok());
  }
}
