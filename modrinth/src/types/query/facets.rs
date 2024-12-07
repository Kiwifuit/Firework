use crate::types::Loader;
use crate::types::ProjectType;
use serde::Serialize;

#[derive(Debug)]
/// Represents the facets used by
/// Modrinth's Search API to filter
/// results.
///
/// More information on how this,
/// and the `Custom` variant works
/// can be found [here](https://docs.modrinth.com/#tag/projects)
pub enum Facet {
  /// Project must be of type...
  ProjectType(ProjectType),
  /// Project must be categorized as...
  Category(String),
  /// Project must be loadable by...
  Loader(Loader),
  /// Project must be supported by minecraft version...
  Version(String),
  /// Project must be open source?
  OpenSource(bool),
  /// Project must be licensed under...
  License(String),
  Custom {
    _type: String,
    op: FacetOp,
    value: String,
  },
}

impl ToString for Facet {
  fn to_string(&self) -> String {
    match self {
      Self::ProjectType(project_type) => format!("project_type:{}", project_type.to_string()),
      Self::Category(category) => format!("categories:{}", category),
      Self::Loader(loader) => format!("categories:{}", loader.to_string()),
      Self::Version(version) => format!("version:{}", version),
      Self::OpenSource(open_source) => format!("open_source:{}", open_source),
      Self::License(license) => format!("license:{}", license),
      Self::Custom { _type, op, value } => format!("{}{}{}", _type, op.to_string(), value),
    }
  }
}

impl Serialize for Facet {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let facet_str = self.to_string();

    serializer.serialize_str(&facet_str)
  }
}

#[derive(Debug)]
pub enum FacetOp {
  Equal,        // =
  NotEqual,     // !=
  GreaterEqual, //  >=
  GreaterThan,  //  >
  LesserEqual,  //  <=
  LesserThan,   //  <
}

impl ToString for FacetOp {
  fn to_string(&self) -> String {
    match self {
      Self::Equal => "=",
      Self::NotEqual => "!=",
      Self::GreaterEqual => ">=",
      Self::GreaterThan => ">",
      Self::LesserEqual => "<=",
      Self::LesserThan => "<",
    }
    .to_string()
  }
}
