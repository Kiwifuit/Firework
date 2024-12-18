use super::{Gallery, License, ModRequirement, ModrinthProjectMeta, ProjectType};
use serde::Deserialize;
use std::rc::Rc;

#[derive(Debug, Deserialize)]
pub struct SearchProjectResult {
  pub hits: Rc<[SearchProjectHit]>,
  pub offset: u8,
  pub limit: u8,
  pub total_hits: u16,
}

#[derive(Debug, Deserialize)]
/// Represents a search result returned by Modrinth.
///
/// *The documentation for this struct's fields have*
/// *been copied over from [Modrinth's documentation](https://docs.modrinth.com/#tag/project_result_model)*
pub struct SearchProjectHit {
  /// The slug of a project, used for vanity URLs. Regex: `^[\w!@$()`.+,"\-']{3,64}$`
  pub slug: Rc<str>,
  /// The title or name of the project
  pub title: Rc<str>,
  /// A short description of the project
  pub description: Rc<str>,
  /// A list of the categories that the project has
  pub categories: Vec<Rc<str>>,
  /// The client side support of the project
  pub client_side: ModRequirement,
  /// The server side support of the project
  pub server_side: ModRequirement,
  /// The project type of the project
  pub project_type: ProjectType,
  /// The total number of downloads of the project
  pub downloads: u32,
  /// The URL of the project's icon
  pub icon_url: Rc<str>,
  /// The RGB color of the project, automatically generated from the project icon
  pub color: u32,
  /// The ID of the project
  pub project_id: Rc<str>,
  /// The username of the project's author
  pub author: Rc<str>,
  /// A list of the minecraft versions supported by the project
  pub versions: Vec<Rc<str>>,
  /// The date the project was added to search
  pub date_created: Rc<str>,
  /// The date the project was last modified
  pub date_modified: Rc<str>,
  /// The latest version of minecraft that this project supports
  pub latest_version: Rc<str>,
  /// The SPDX license ID of a project
  pub license: License,
  /// All gallery images attached to the project
  pub gallery: Gallery,
  /// The featured gallery image of the project
  pub featured_gallery: Option<Rc<str>>,
}

impl ModrinthProjectMeta for SearchProjectHit {
  type Id = Rc<str>;

  fn project_id(&self) -> Option<Self::Id> {
    Some(self.project_id.clone())
  }
}

impl ModrinthProjectMeta for &SearchProjectHit {
  type Id = Rc<str>;

  fn project_id(&self) -> Option<Self::Id> {
    Some(self.project_id.clone())
  }
}
