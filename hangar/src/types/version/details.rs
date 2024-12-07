use super::HangarPlatform;
use serde::Deserialize;

use std::rc::Rc;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HPDownloadDetails {
  pub file_info: HangarVersionDownloadFile,
  pub external_url: Option<Rc<str>>,
  pub download_url: Option<Rc<str>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HPPluginDependencyDetails {
  pub required: bool,
  pub external_url: Option<Rc<str>>,
  pub platform: HangarPlatform,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HangarVersionDownloadFile {
  pub name: Rc<str>,
  pub size_bytes: usize,
  pub sha_256_hash: Rc<str>,
}
