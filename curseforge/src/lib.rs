use std::{env, time::Duration};

use anyhow::Context;
use log::debug;
use log::info;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::Client;
use thiserror::Error;
use types::project::CurseMod;
use types::CurseResponse;

pub mod types;

pub(crate) const CURSE_API: &str = "https://api.curseforge.com/v1";
pub(crate) const CURSE_MINECRAFT_ID: u16 = 432;
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
  ) -> Result<CurseResponse<Vec<CurseMod>>, CurseClientError> {
    info!("Fetching mod {:?} from curseforge", query.search_filter);

    let url = format!("{}/mods/search", CURSE_API);
    let req = self.client.get(url).query(query);

    debug!("url:\n{:#?}", req);

    let resp = req
      .send()
      .await?
      .json::<CurseResponse<Vec<CurseMod>>>()
      .await?;

    info!("Got {} mod(s)", resp.pagination.result_count);

    Ok(resp)
  }
}
