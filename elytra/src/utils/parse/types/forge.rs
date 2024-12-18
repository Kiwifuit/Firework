use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;

use serde::de::{Deserializer, MapAccess, Visitor};
use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize, Debug)]
pub struct ForgeMod {
  #[serde(rename = "modLoader")]
  pub mod_loader: Rc<str>,
  #[serde(rename = "loaderVersion")]
  pub loader_version: ForgeModVersion,
  pub license: Rc<str>,
  #[serde(rename = "issueTrackerURL")]
  pub issue_tracker: Option<Rc<str>>,
  #[serde(rename = "displayURL")]
  pub homepage_url: Option<Rc<str>>,
  pub mods: Rc<[ForgeModMetadata]>,
  pub dependencies: Option<HashMap<Rc<str>, Rc<[ForgeModDependency]>>>,
}

#[derive(Deserialize, Debug)]
pub struct ForgeModMetadata {
  #[serde(rename = "modId")]
  pub id: Rc<str>,
  pub version: Rc<str>,
  #[serde(rename = "displayName")]
  pub display_name: Rc<str>,
  pub authors: Option<ForgeModAuthors>,
  pub credits: Option<Rc<str>>,
  pub description: Rc<str>,
  #[serde(rename = "updateJSONURL")]
  pub update_url: Option<Rc<str>>,
  #[serde(rename = "displayURL")]
  pub homepage_url: Option<Rc<str>>,
  #[serde(rename = "logoFile")]
  pub logo: Option<PathBuf>,
}

#[derive(Deserialize, Debug)]
pub struct ForgeModDependency {
  #[serde(rename = "modId")]
  pub id: Rc<str>,
  // pub version: ModVersion,
  pub mandatory: bool,
  #[serde(rename = "versionRange")]
  pub version_range: ForgeModVersion,
  pub ordering: Option<Rc<str>>,
  pub side: Rc<str>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ModSemver {
  pub major: Option<u32>,
  pub minor: Option<u32>,
  pub patch: Option<u32>,
}

#[derive(Debug)]
pub struct ModVersionRange {
  pub from: ModSemver,
  pub to: Option<ModSemver>,
  pub mode: ModVersionRangeMode,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ForgeModAuthors {
  SingleAuthor(String),
  MultipleAuthors(Vec<String>),
}

#[derive(Debug)]
pub enum ForgeModVersion {
  Any,
  VersionRange(ModVersionRange),
  SpecificVersion(ModSemver),
}

impl<'de> Deserialize<'de> for ForgeModVersion {
  fn deserialize<D>(deserializer: D) -> Result<ForgeModVersion, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct ModLoeaderVersionVisitor;

    impl<'de> Visitor<'de> for ModLoeaderVersionVisitor {
      type Value = ForgeModVersion;

      fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("valid string or struct representing ModLoeaderVersion variant")
      }

      fn visit_str<E>(self, value: &str) -> Result<ForgeModVersion, E>
      where
        E: serde::de::Error,
      {
        match value {
          "*" => Ok(ForgeModVersion::Any),
          version if version.chars().nth(0).unwrap().is_numeric() => Ok(
            ForgeModVersion::SpecificVersion(version.parse().map_err(serde::de::Error::custom)?),
          ),
          version => Ok(if version.starts_with('[') {
            ForgeModVersion::VersionRange(version.parse().map_err(serde::de::Error::custom)?)
          } else {
            ForgeModVersion::SpecificVersion(version.parse().map_err(serde::de::Error::custom)?)
          }),
        }
      }

      fn visit_map<A>(self, mut access: A) -> Result<ForgeModVersion, A::Error>
      where
        A: MapAccess<'de>,
      {
        let my_struct =
          ModVersionRange::deserialize(serde::de::value::MapAccessDeserializer::new(&mut access))?;
        Ok(ForgeModVersion::VersionRange(my_struct))
      }
    }

    deserializer.deserialize_any(ModLoeaderVersionVisitor)
  }
}

impl<'de> Deserialize<'de> for ModSemver {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let mod_data = String::deserialize(deserializer)?;
    mod_data.parse().map_err(serde::de::Error::custom)
  }
}

impl FromStr for ModSemver {
  type Err = ModVersionParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let a = s
      .split('.')
      .map(|component| {
        component
          .parse::<u32>()
          .map_err(ModVersionParseError::Parse)
        // .map(|num| if num == 0 { None } else { Some(num) })
      })
      .collect::<Vec<Result<u32, ModVersionParseError>>>();

    Ok(ModSemver {
      major: a.first().and_then(|some_case| some_case.to_owned().ok()),
      minor: a.get(1).and_then(|some_case| some_case.to_owned().ok()),
      patch: a.get(2).and_then(|some_case| some_case.to_owned().ok()),
    })
  }
}

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum ModVersionParseError {
  #[error("error while parsing version: {0}")]
  Parse(#[from] std::num::ParseIntError),
}

impl<'de> Deserialize<'de> for ModVersionRange {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let mod_data = String::deserialize(deserializer)?;
    mod_data.parse().map_err(serde::de::Error::custom)
  }
}

impl ModVersionRange {
  fn is_infinity(version: &Option<ModSemver>) -> bool {
    if let Some(max_version) = version {
      max_version.major.is_none() && max_version.minor.is_none() && max_version.patch.is_none()
    } else {
      true
    }
  }
}

impl FromStr for ModVersionRange {
  type Err = ModVersionRangeParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let delimeter_loc = s.find(',');
    let closing_loc = s.find(']').or(s.find(')'));
    let mut is_strict_version = false;

    if delimeter_loc.is_none() && closing_loc.is_none() {
      // we assume that we will find a comma somewhere
      return Err(ModVersionRangeParseError::Malformed(s.to_string()));
    } else if delimeter_loc.is_none() && closing_loc.is_some() {
      // we assume that we are given a 'strict version requirement'
      // e.g. STRICTLY 1.19.2 and no other version
      is_strict_version = true;
    } else if delimeter_loc.unwrap() == 1 {
      // we assume that we will find a minimum version at the beginning
      return Err(ModVersionRangeParseError::NoMinimum);
    }

    if closing_loc.is_none() {
      // we assume that we will find a closing `]` or `)` somewhere
      return Err(ModVersionRangeParseError::Unclosed);
    }

    let delimeter_loc = delimeter_loc.unwrap_or(closing_loc.unwrap());
    let strlen = s.len();

    let ver_min = s[1..delimeter_loc].parse::<ModSemver>();
    let ver_max = if is_strict_version {
      None
    } else {
      s[delimeter_loc + 1..strlen - 1].parse::<ModSemver>().ok()
    };
    let mode = if is_strict_version {
      ModVersionRangeMode::None
    } else if Self::is_infinity(&ver_max) {
      ModVersionRangeMode::GreaterThan
    } else {
      match s.chars().nth(closing_loc.unwrap()).unwrap() {
        ')' => ModVersionRangeMode::Between,
        ']' => ModVersionRangeMode::BetweenInclusive,
        _ => ModVersionRangeMode::None,
      }
    };

    Ok(ModVersionRange {
      from: ver_min?,
      to: ver_max,
      mode,
    })
  }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ModVersionRangeParseError {
  #[error("string {0:?} is malformed")]
  Malformed(String),

  #[error("string does not supply a minimum version")]
  NoMinimum,

  #[error("expected `]` or `)` from string, found none")]
  Unclosed,

  #[error("unable to parse mod version: {0}")]
  Parse(#[from] ModVersionParseError),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ModVersionRangeMode {
  // "any version greater than or equal to a"
  GreaterThan,
  // // "any version lesser than or equal to a"
  // LesserThan,

  // TODO: Give this a better name
  // "any version between a and b, including a and b"
  BetweenInclusive,

  // "any version between a and b, including a but excluding b"
  Between,

  // specifically version a
  None,
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::unzip::grab_meta_file;
  use std::fs::read_dir;
  use toml::from_str;

  #[test]
  fn mod_version() {
    let version_raw = "1.2.3";

    let version = version_raw.parse::<ModSemver>();
    assert!(version.is_ok());

    let version_parsed = version.unwrap();

    assert_eq!(version_parsed.major, Some(1));
    assert_eq!(version_parsed.minor, Some(2));
    assert_eq!(version_parsed.patch, Some(3));
  }

  #[test]
  fn mod_version_major() {
    let version_raw = "1";

    let version = version_raw.parse::<ModSemver>();
    assert!(version.is_ok());

    let version_parsed = version.unwrap();

    assert_eq!(version_parsed.major, Some(1));
    assert_eq!(version_parsed.minor, None);
    assert_eq!(version_parsed.patch, None);
  }

  #[test]
  fn mod_version_major_minor() {
    let version_raw = "1.2";

    let version = version_raw.parse::<ModSemver>();
    assert!(version.is_ok());

    let version_parsed = version.unwrap();

    assert_eq!(version_parsed.major, Some(1));
    assert_eq!(version_parsed.minor, Some(2));
    assert_eq!(version_parsed.patch, None);
  }

  #[test]
  fn mod_version_major_minor_patch() {
    let version_raw = "1.2.3";

    let version = version_raw.parse::<ModSemver>();
    assert!(version.is_ok());

    let version_parsed = version.unwrap();

    assert_eq!(version_parsed.major, Some(1));
    assert_eq!(version_parsed.minor, Some(2));
    assert_eq!(version_parsed.patch, Some(3));
  }

  #[test]
  fn version_range_unclosed() {
    let version = "[1.2.3,";

    let mod_version = version.parse::<ModVersionRange>();
    assert!(mod_version.is_err());
    assert_eq!(
      mod_version.unwrap_err(),
      ModVersionRangeParseError::Unclosed
    )
  }

  #[test]
  fn version_range_malformed() {
    let version = "[1.2.3";

    let mod_version = version.parse::<ModVersionRange>();
    assert!(mod_version.is_err());
    assert_eq!(
      mod_version.unwrap_err(),
      ModVersionRangeParseError::Malformed("[1.2.3".to_string())
    )
  }

  #[test]
  fn version_range_no_minimum() {
    let version = "[,)";

    let mod_version = version.parse::<ModVersionRange>();
    assert!(mod_version.is_err());
    assert_eq!(
      mod_version.unwrap_err(),
      ModVersionRangeParseError::NoMinimum
    )
  }

  #[test]
  fn version_range_shouldnt_fail() {
    let version = "[1.2.3,4.5.6)";

    let mod_version = version.parse::<ModVersionRange>();
    assert!(mod_version.is_ok());

    let mod_version = mod_version.unwrap();

    assert_eq!(
      mod_version.from,
      ModSemver {
        major: Some(1),
        minor: Some(2),
        patch: Some(3)
      }
    );

    assert_eq!(
      mod_version.to,
      Some(ModSemver {
        major: Some(4),
        minor: Some(5),
        patch: Some(6)
      })
    );

    assert_eq!(mod_version.mode, ModVersionRangeMode::Between)
  }

  #[test]
  fn version_range_shouldnt_fail_inclusive() {
    let version = "[1.2.3,4.5.6]";

    let mod_version = version.parse::<ModVersionRange>();
    assert!(mod_version.is_ok());

    let mod_version = mod_version.unwrap();

    assert_eq!(
      mod_version.from,
      ModSemver {
        major: Some(1),
        minor: Some(2),
        patch: Some(3)
      }
    );

    assert_eq!(
      mod_version.to,
      Some(ModSemver {
        major: Some(4),
        minor: Some(5),
        patch: Some(6)
      })
    );

    assert_eq!(mod_version.mode, ModVersionRangeMode::BetweenInclusive)
  }

  #[test]
  fn version_range_shouldnt_fail_single_inclusive() {
    let version = "[1.2.3]";

    let mod_version = version.parse::<ModVersionRange>();
    assert!(mod_version.is_ok());

    let mod_version = mod_version.unwrap();

    assert_eq!(
      mod_version.from,
      ModSemver {
        major: Some(1),
        minor: Some(2),
        patch: Some(3)
      }
    );

    assert_eq!(mod_version.to, None);
    assert_eq!(mod_version.mode, ModVersionRangeMode::None)
  }

  #[test]
  fn version_range_shouldnt_fail_greater() {
    let version = "[1.2.3,)";

    let mod_version = version.parse::<ModVersionRange>();
    assert!(mod_version.is_ok());

    let mod_version = mod_version.unwrap();

    assert_eq!(
      mod_version.from,
      ModSemver {
        major: Some(1),
        minor: Some(2),
        patch: Some(3)
      }
    );

    assert_eq!(
      mod_version.to,
      Some(ModSemver {
        major: None,
        minor: None,
        patch: None
      })
    );

    assert_eq!(mod_version.mode, ModVersionRangeMode::GreaterThan)
  }

  #[test]
  fn mod_manifest() {
    for file in read_dir("samples/forge/").unwrap() {
      let file = file.unwrap();

      if file.file_type().unwrap().is_dir() {
        continue;
      }

      let mod_meta = from_str::<ForgeMod>(
        &grab_meta_file(file.path())
          .expect("expected meta file to be grabbed")
          .raw,
      );

      assert!(mod_meta.is_ok());
    }
  }
}
