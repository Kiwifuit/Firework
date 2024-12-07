pub(crate) mod search;
pub(crate) mod version;

pub use search::SearchQueryBuilder;
pub use version::VersionQueryBuilder;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GenericPagination {
  pub(crate) limit: u8,
  pub(crate) offset: u8,
}

impl Default for GenericPagination {
  fn default() -> Self {
    Self {
      limit: 25,
      offset: 0,
    }
  }
}

impl GenericPagination {
  pub fn set_limit(&mut self, limit: u8) {
    self.limit = limit;
  }

  pub fn set_offset(&mut self, offset: u8) {
    self.offset = offset;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_urlencoded::to_string;

  #[test]
  fn pagination_serialization() {
    let pagination = GenericPagination::default();
    let res = to_string(&pagination);

    assert_eq!(&res.unwrap(), "limit=25&offset=0");
  }
}
