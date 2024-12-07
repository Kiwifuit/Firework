use super::ModLoader;
use std::rc::Rc;

use log::{error, warn};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
#[serde(rename = "camelCase")]
pub struct CurseMod {
  pub(crate) id: u32,
  pub logo: CurseAsset,
  pub name: Rc<str>,
  pub links: CurseModLinks,
  pub summary: Rc<str>,
  pub latest_files_indexes: Option<Rc<[CurseFileIndex]>>,
  pub latest_early_access_files_indexes: Option<Rc<[CurseFileIndex]>>,
  pub categories: Rc<[CurseCategory]>,
  pub authors: Rc<[CurseAuthor]>,
  pub screenshots: Rc<[CurseAsset]>,
  #[serde(rename = "allowModDistribution")]
  pub(crate) allowed: Option<bool>,
}

impl CurseMod {
  pub fn is_allowed(&self) -> bool {
    self.allowed.is_some_and(|v| v)
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurseFileIndex {
  // file_id: usize,
  pub game_version: Rc<str>,
  #[serde(rename = "filename")]
  pub file_name: Rc<str>,
  pub release_type: CurseRelease,
  pub game_version_type_id: Option<u32>,
  pub mod_loeader: ModLoader,
}

#[derive(Debug, Deserialize)]
#[repr(u8)]
pub enum CurseRelease {
  Release = 1,
  Beta = 2,
  Alpha = 3,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurseModLinks {
  #[serde(deserialize_with = "unwrap_str")]
  pub website_url: Option<Rc<str>>,
  #[serde(deserialize_with = "unwrap_str")]
  pub wiki_url: Option<Rc<str>>,
  #[serde(deserialize_with = "unwrap_str")]
  pub issues_url: Option<Rc<str>>,
  #[serde(deserialize_with = "unwrap_str")]
  pub source_url: Option<Rc<str>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "camelCase")]
pub struct CurseCategory {
  pub id: u32,
  pub name: Rc<str>,
  pub url: Rc<str>,
  pub icon_url: Option<Rc<str>>,
}

#[derive(Debug, Deserialize)]
pub struct CurseAuthor {
  pub id: u32,
  pub name: Rc<str>,
  pub url: Rc<str>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "camelCase")]
pub struct CurseAsset {
  pub id: u32,

  #[serde(deserialize_with = "unwrap_str")]
  pub title: Option<Rc<str>>,

  #[serde(deserialize_with = "unwrap_str")]
  pub description: Option<Rc<str>>,

  // #[serde(deserialize_with = "unwrap_str")]
  pub thumbnail_url: Option<Rc<str>>,

  #[serde(deserialize_with = "unwrap_str")]
  pub url: Option<Rc<str>>,
}

fn unwrap_str<'de, D>(deserializer: D) -> Result<Option<Rc<str>>, D::Error>
where
  D: Deserializer<'de>,
{
  let raw = Option::deserialize(deserializer);

  if let Err(err) = &raw {
    error!("String cannot be deserialized: {}", err);
    warn!("This function will return `None` as default");

    return Ok(None);
  }

  let url: Option<String> = raw.ok().flatten();
  let a = url.filter(|u| !u.is_empty()).map(|s| Rc::from(s.as_str()));

  Ok(a)
}
