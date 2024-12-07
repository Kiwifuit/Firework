use super::GenericPagination;
use crate::types::HangarPlatform;
use serde::Serialize;

use std::rc::Rc;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionQuery {
  #[serde(flatten)]
  pub(crate) pagination: GenericPagination,
  pub(crate) include_hidden_channels: bool,
  pub(crate) platform: HangarPlatform,
  #[serde(skip_serializing_if = "str::is_empty")]
  pub(crate) platform_version: Rc<str>,
}

#[derive(Debug, Default)]
pub struct VersionQueryBuilder {
  pagination: Option<GenericPagination>,
  include_hidden_channels: Option<bool>,
  platform: Option<HangarPlatform>,
  platform_version: Option<Rc<str>>,
}

impl VersionQueryBuilder {
  pub fn pagination(mut self, pagination: GenericPagination) -> Self {
    self.pagination = Some(pagination);

    self
  }

  pub fn include_hidden_channels(mut self, include_hidden_channels: bool) -> Self {
    self.include_hidden_channels = Some(include_hidden_channels);

    self
  }

  pub fn platform(mut self, platform: HangarPlatform) -> Self {
    self.platform = Some(platform);

    self
  }

  pub fn platform_version(mut self, platform_version: Rc<str>) -> Self {
    self.platform_version = Some(platform_version);

    self
  }

  pub fn build(self) -> VersionQuery {
    VersionQuery {
      include_hidden_channels: self.include_hidden_channels.unwrap_or(true),
      pagination: self.pagination.unwrap_or_default(),
      platform: self.platform.unwrap_or_default(),
      platform_version: self.platform_version.unwrap_or_default(),
    }
  }
}
