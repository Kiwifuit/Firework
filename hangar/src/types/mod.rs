use std::fmt::Display;

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

pub mod project;
pub mod query;
pub mod version;

pub use project::HangarProjects;
pub use version::HangarVersions;

type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HangarVisibility {
  Public,
  New,
  NeedsChanges,
  NeedsApproval,
  SoftDelete,
}

#[derive(Debug, Default, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum HangarPlatform {
  #[default]
  Paper,
  Waterfall,
  Velocity,
}

impl Display for HangarPlatform {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Paper => "PAPER",
        Self::Waterfall => "WATERFALL",
        Self::Velocity => "VELOCITY",
      }
    )
  }
}

bitflags! {
    #[derive(Debug, Serialize, Default)]
    pub struct HangarTags: u8 {
        const ADDON          = 1;
        const LIBRARY        = 2;
        const SUPPORTS_FOLIA = 3;
    }
}

impl<'de> Deserialize<'de> for HangarTags {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let tags: Vec<String> = Vec::deserialize(deserializer)?;
    let mut flags = Self::empty();

    for tag in tags {
      match tag.as_str() {
        "ADDON" => flags |= Self::ADDON,
        "LIBRARY" => flags |= Self::LIBRARY,
        "SUPPORTS_FOLIA" => flags |= Self::SUPPORTS_FOLIA,
        other => {
          return Err(serde::de::Error::unknown_variant(
            other,
            &["ADDON", "LIBRARY", "SUPPORTS_FOLIA"],
          ))
        }
      }
    }

    Ok(flags)
  }
}
