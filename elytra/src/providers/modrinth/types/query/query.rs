use crate::types::Facet;
use crate::types::IndexBy;
use serde::Serialize;

#[derive(Debug, Serialize)]
/// Represents a built complex search query for
/// `search_projects`
pub struct ProjectQuery {
  pub(crate) query: String,
  #[serde(
    skip_serializing_if = "Vec::is_empty",
    serialize_with = "crate::types::serialize_vec_nested"
  )]
  pub(crate) facets: Vec<Vec<Facet>>,
  // TODO: some sort of is_default thingy
  //       so that serde omits this if its
  //       set to its defaults
  pub(crate) index: IndexBy,
  #[serde(skip_serializing_if = "crate::types::is_zero")]
  pub(crate) offset: u8,
  #[serde(skip_serializing_if = "crate::types::is_zero")]
  pub(crate) limit: u8,
}
#[derive(Debug, Default)]
/// Represents a complex search query for
/// `search_projects`. Use `.build()` to build
/// the query
pub struct ProjectQueryBuilder {
  query: Option<String>,
  facets: Option<Vec<Vec<Facet>>>,
  index_by: Option<IndexBy>,
  offset: Option<u8>,
  limit: Option<u8>,
}

impl ProjectQueryBuilder {
  /// Creates a new query
  pub fn new() -> Self {
    Self::default()
  }

  /// The project to search for
  pub fn query<S: ToString>(mut self, query: S) -> Self {
    self.query = Some(query.to_string());
    self
  }

  /// Facets are an essential concept for understanding how to filter out results.
  /// These are the most commonly used facet types:
  ///
  ///   - `project_type`
  ///   - `categories` (loaders are lumped in with categories in search)
  ///   - `versions`
  ///   - `client_side`
  ///   - `server_side`
  ///   - `open_source`
  ///
  /// Several others are also available for use, though these should not be used outside very specific use cases.
  ///
  ///   - `title`
  ///   - `author`
  ///   - `follows`
  ///   - `project_id`
  ///   - `license`
  ///   - `downloads`
  ///   - `color`
  ///   - `created_timestamp`
  ///   - `modified_timestamp`
  ///
  /// In order to then use these facets, you need a value to filter by, as well as an operation to perform on this value. The most common operation is : (same as =), though you can also use !=, >=, >, <=, and <. Join together the type, operation, and value, and you've got your string.
  /// > `{type} {operation} {value}`
  ///
  /// Examples:
  /// - `categories = adventure`
  /// - `versions != 1.20.1`
  /// - `downloads <= 100`
  ///
  /// You then join these strings together in arrays to signal AND and OR operators.
  /// ### OR
  ///
  /// All elements in a single array are considered to be joined by OR statements.
  /// For example, the search `[["versions:1.16.5", "versions:1.17.1"]]` translates to *Projects that support 1.16.5 OR 1.17.1.*
  /// ### AND
  ///
  /// Separate arrays are considered to be joined by AND statements.
  /// For example, the search `[["versions:1.16.5"], ["project_type:modpack"]]` translates to *Projects that support 1.16.5 AND are modpacks*.
  pub fn facets(mut self, facets: Vec<Vec<Facet>>) -> Self {
    self.facets = Some(facets);
    self
  }

  /// TThe offset into the search. Skips this number of results
  pub fn offset(mut self, offset: u8) -> Self {
    self.offset = Some(offset);
    self
  }

  /// The sorting method used for sorting search results
  pub fn index_by(mut self, index_by: IndexBy) -> Self {
    self.index_by = Some(index_by);
    self
  }

  /// The number of results returned by the search
  ///
  /// # Disclaimer
  /// This function silently does nothing if the supplied
  /// `limit` is above 100 in accordance to modrinth's limits
  pub fn limit(mut self, limit: u8) -> Self {
    self.limit = limit.lt(&100).then_some(limit);

    self
  }

  /// Builds the query
  pub fn build(self) -> ProjectQuery {
    ProjectQuery {
      query: self.query.unwrap_or_default(),
      facets: self.facets.unwrap_or_default(),
      index: self.index_by.unwrap_or_default(),
      offset: self.offset.unwrap_or_default(),
      limit: self.limit.unwrap_or(10),
    }
  }
}
