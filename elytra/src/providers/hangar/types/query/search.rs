use super::GenericPagination;
use crate::types::{HangarPlatform, HangarTags};
use serde::Serialize;

use std::rc::Rc;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchQuery {
  pub(crate) prioritize_exact_match: bool,
  #[serde(flatten)]
  pub(crate) pagination: GenericPagination,
  pub(crate) sort: SortBy,
  #[serde(skip_serializing_if = "str::is_empty")]
  pub(crate) category: Rc<str>,
  pub(crate) platform: HangarPlatform,
  #[serde(skip_serializing_if = "str::is_empty")]
  pub(crate) owner: Rc<str>,
  #[serde(skip_serializing_if = "str::is_empty")]
  pub(crate) query: Rc<str>,
  #[serde(skip_serializing_if = "str::is_empty")]
  pub(crate) license: Rc<str>,
  #[serde(skip_serializing_if = "str::is_empty")]
  pub(crate) version: Rc<str>,
  #[serde(skip_serializing_if = "HangarTags::is_empty")]
  pub(crate) tag: HangarTags,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SortBy {
  Views,
  #[default]
  Downloads,
  Newest,
  Stars,
  Updated,
  RecentDownloads,
  RecentViews,
  Slugs,
}

#[derive(Debug, Default)]
pub struct SearchQueryBuilder {
  prioritize_exact_match: Option<bool>,
  pagination: Option<GenericPagination>,
  sort: Option<SortBy>,
  category: Option<Rc<str>>,
  platform: Option<HangarPlatform>,
  owner: Option<Rc<str>>,
  query: Option<Rc<str>>,
  license: Option<Rc<str>>,
  version: Option<Rc<str>>,
  tag: Option<HangarTags>,
}

impl SearchQueryBuilder {
  pub fn prioritize_exact_match(mut self, prioritize_exact_match: bool) -> Self {
    self.prioritize_exact_match = Some(prioritize_exact_match);

    self
  }

  pub fn pagination(mut self, pagination: GenericPagination) -> Self {
    self.pagination = Some(pagination);

    self
  }

  pub fn sort(mut self, sort: SortBy) -> Self {
    self.sort = Some(sort);

    self
  }

  pub fn category<T: ToString>(mut self, category: T) -> Self {
    self.category = Some(Rc::from(category.to_string().into_boxed_str()));

    self
  }

  pub fn platform(mut self, platform: HangarPlatform) -> Self {
    self.platform = Some(platform);

    self
  }

  pub fn owner<T: ToString>(mut self, owner: T) -> Self {
    self.owner = Some(Rc::from(owner.to_string().into_boxed_str()));

    self
  }

  pub fn query<T: ToString>(mut self, query: T) -> Self {
    self.query = Some(Rc::from(query.to_string().into_boxed_str()));

    self
  }

  pub fn license<T: ToString>(mut self, license: T) -> Self {
    self.license = Some(Rc::from(license.to_string().into_boxed_str()));

    self
  }

  pub fn version<T: ToString>(mut self, version: T) -> Self {
    self.version = Some(Rc::from(version.to_string().into_boxed_str()));

    self
  }

  pub fn tag(mut self, tag: HangarTags) -> Self {
    self.tag = Some(tag);

    self
  }

  pub fn build(self) -> SearchQuery {
    SearchQuery {
      prioritize_exact_match: self.prioritize_exact_match.unwrap_or(true),
      pagination: self.pagination.unwrap_or_default(),
      sort: self.sort.unwrap_or_default(),
      category: self.category.unwrap_or_default(),
      platform: self.platform.unwrap_or_default(),
      owner: self.owner.unwrap_or_default(),
      query: self.query.unwrap_or_default(),
      license: self.license.unwrap_or_default(),
      version: self.version.unwrap_or_default(),
      tag: self.tag.unwrap_or_default(),
    }
  }
}
