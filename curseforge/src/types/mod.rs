use std::ops::Deref;

use serde::{Deserialize, Serialize};

pub mod project;
pub mod query;

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[repr(u8)]
pub enum ModLoader {
  #[default]
  Any = 0,
  Forge = 1,
  Cauldron = 2,
  LiteLoader = 3,
  Fabric = 4,
  Quilt = 5,
  NeoForge = 6,
}

impl ToString for ModLoader {
  fn to_string(&self) -> String {
    (*self as u8).to_string()
  }
}

#[derive(Debug)]
pub struct CurseResponse<T> {
  inner: T,
  pub pagination: CursePagination,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CursePagination {
  pub index: u8,
  pub page_size: u8,
  pub result_count: u8,
  pub total_count: u8,
}

impl<T> Deref for CurseResponse<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl<T> CurseResponse<T> {
  pub fn new(inner: T) -> Self {
    Self {
      inner,
      pagination: CursePagination::default(),
    }
  }
}

impl<'de, T> Deserialize<'de> for CurseResponse<T>
where
  T: Deserialize<'de>,
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let mut data = serde_json::Map::deserialize(deserializer)?;
    let inner = data
      .remove("data")
      .ok_or_else(|| serde::de::Error::missing_field("data"))
      .and_then(T::deserialize)
      .map_err(serde::de::Error::custom)?;

    let pagination = data
      .remove("pagination")
      .ok_or_else(|| serde::de::Error::missing_field("pagination"))
      .and_then(serde_json::from_value)
      .map_err(serde::de::Error::custom)?;

    Ok(Self { inner, pagination })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde::Deserialize;

  #[derive(Debug, Deserialize)]
  struct TestStruct {
    name: String,
  }

  #[test]
  fn test_cresp() {
    let json_data = r#"
      {
        "data": {
          "name": "Samuel L Jackson"
        },
        "pagination": {
          "index": 0,
          "pageSize": 5,
          "resultCount": 5,
          "totalCount": 15
        }
      }
    "#;

    let data = serde_json::from_str::<CurseResponse<TestStruct>>(json_data);

    assert!(data.is_ok_and(|d| d.name == "Samuel L Jackson"))
  }
}
