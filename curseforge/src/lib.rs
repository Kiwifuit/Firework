use std::{env, time::Duration};

use anyhow::Context;
use log::info;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::Client;
use thiserror::Error;

pub mod types;

const CURSE_API: &str = "https://api.curseforge.com/v1";

pub struct CurseForge {
  client: Client,
}

#[derive(Debug, Error)]
pub enum CurseClientError {
  #[error("http error: {0}")]
  Http(#[from] reqwest::Error),

  #[error("header error while building curseforge client: {0}")]
  Header(#[from] reqwest::header::InvalidHeaderValue),

  #[error("no environment variable `CURSE_API_KEY` supplied: {0}")]
  NoApiKey(#[from] std::env::VarError),
}

impl CurseForge {
  pub fn new(api_key: &str) -> Result<Self, CurseClientError> {
    info!("Building CurseForge Client");
    let mut headers = HeaderMap::new();

    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers.insert("X-Api-Key", HeaderValue::from_str(api_key)?);

    info!("{:?}", headers);

    Ok(Self {
      client: Client::builder()
        .default_headers(headers)
        .build()
        .expect("expected client to be built"),
    })
  }

  pub fn new_from_env() -> Result<Self, CurseClientError> {
    Self::new(&env::var("CURSE_API_KEY")?)
  }

  pub async fn get_mod(
    &self,
    query: &types::query::ModQueryBuilder,
  ) -> Result<(), CurseClientError> {
    info!("Fetching mod {:?} from curseforge", query.search_filter);

    let url = format!("{}/mods/search", CURSE_API);
    let req = self
      .client
      .get(url)
      .query(query)
      .send()
      .await?
      .json()
      .await?;

    Ok(req)
  }
}
