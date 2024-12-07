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
  //   #[serde(rename = "displayURL")]
  //   pub homepage_url: Option<Rc<str>>,
  pub mods: Rc<[ForgeModMetadata]>,
  pub dependencies: Option<HashMap<Rc<str>, Rc<[ForgeModDependency]>>>,
}

#[derive(Deserialize, Debug)]
pub struct ForgeModMetadata {
  #[serde(rename = "modId")]
  pub id: Rc<str>,
  pub version: ModSemver,
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
  //   pub ordering: Option<Rc<str>>,
  //   pub side: Option<Rc<str>>,
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
  Custom(Rc<str>),
  VersionRange(ModVersionRange),
  SpecificVersion(ModSemver),
}

impl ModVersionRange {
  fn within_range(&self, other: &ModSemver) -> bool {
    match self.mode {
      ModVersionRangeMode::None => {
        panic!("this is a weird case. `ModVersionRange.mode` is `None`!")
      }
      ModVersionRangeMode::Between => {
        self.from < *other
          && if let Some(to) = &self.to {
            other < to
          } else {
            true
          }
      }
      ModVersionRangeMode::BetweenInclusive => {
        self.from <= *other
          && if let Some(to) = &self.to {
            other <= to
          } else {
            true
          }
      }
      ModVersionRangeMode::GreaterThan => self.from < *other,
    }
  }
}

impl ForgeModVersion {
  pub fn satisfies(&self, version: &ModSemver) -> bool {
    match &self {
      ForgeModVersion::Any => true,
      ForgeModVersion::Custom(version) => !version.is_empty(),
      ForgeModVersion::SpecificVersion(this_version) => this_version == version,
      ForgeModVersion::VersionRange(range) => range.within_range(version),
    }
  }
}

impl<'de> Deserialize<'de> for ForgeModVersion {
  fn deserialize<D>(deserializer: D) -> Result<ForgeModVersion, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct ModLoaderVersionVisitor;

    impl<'de> Visitor<'de> for ModLoaderVersionVisitor {
      type Value = ForgeModVersion;

      fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("valid string or struct representing ModLoaderVersion variant")
      }

      fn visit_str<E>(self, value: &str) -> Result<ForgeModVersion, E>
      where
        E: serde::de::Error,
      {
        if value == "*" {
          return Ok(ForgeModVersion::Any);
        }

        if value.starts_with('[') {
          return Ok(ForgeModVersion::VersionRange(
            value.parse().map_err(serde::de::Error::custom)?,
          ));
        }

        Ok(if let Ok(parsed_value) = value.parse() {
          ForgeModVersion::SpecificVersion(parsed_value)
        } else {
          ForgeModVersion::Custom(Rc::from(value))
        })

        // match value {
        //   "*" => Ok(ForgeModVersion::Any),
        //   version
        //     if version
        //       .chars()
        //       .nth(0)
        //       .ok_or(serde::de::Error::custom(
        //         "expected version string to not be empty (version.char().nth(0) returned None)",
        //       ))?
        //       .is_numeric() =>
        //   {
        //     Ok(ForgeModVersion::SpecificVersion(
        //       version.parse().map_err(serde::de::Error::custom)?,
        //     ))
        //   }
        //   version => Ok(if version.starts_with('[') {
        //     ForgeModVersion::VersionRange(version.parse().map_err(serde::de::Error::custom)?)
        //   } else {
        //     ForgeModVersion::SpecificVersion(version.parse().map_err(serde::de::Error::custom)?)
        //   }),
        // }
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

    deserializer.deserialize_any(ModLoaderVersionVisitor)
  }
}

impl<'de> Deserialize<'de> for ModSemver {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let mut mod_data = String::deserialize(deserializer)?;

    assert!(!mod_data.is_empty());
    let processed_mod_data = mod_data
      .contains("-")
      .then(|| {
        let sep_char = mod_data
          .find("-")
          .expect("expected a `-` to exist since this passed");

        mod_data.split_off(sep_char)
      })
      .unwrap_or(mod_data);

    processed_mod_data.parse().map_err(serde::de::Error::custom)
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
      })
      .collect::<Vec<_>>();

    Ok(ModSemver {
      major: a.first().map_or(Ok(None), |res| match res {
        Ok(num) => Ok(Some(*num)),
        Err(err) => Err(err.clone()),
      })?,

      minor: a.get(1).map_or(Ok(None), |res| match res {
        Ok(num) => Ok(Some(*num)),
        Err(err) => Err(err.clone()),
      })?,

      patch: a.get(2).map_or(Ok(None), |res| match res {
        Ok(num) => Ok(Some(*num)),
        Err(err) => Err(err.clone()),
      })?,
    })
  }
}

impl Ord for ModSemver {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self
      .major
      .unwrap_or_default()
      .cmp(&other.major.unwrap_or_default())
      .then(
        self
          .minor
          .unwrap_or_default()
          .cmp(&other.minor.unwrap_or_default()),
      )
      .then(
        self
          .patch
          .unwrap_or_default()
          .cmp(&other.patch.unwrap_or_default()),
      )
  }
}

impl PartialOrd for ModSemver {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
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
    let delimiter_loc = s.find(',');
    let closing_loc = s.find(']').or(s.find(')'));
    let mut is_strict_version = false;

    if delimiter_loc.is_none() && closing_loc.is_none() {
      // we assume that we will find a comma somewhere
      return Err(ModVersionRangeParseError::Malformed(s.to_string()));
    } else if delimiter_loc.is_none() && closing_loc.is_some() {
      // we assume that we are given a 'strict version requirement'
      // e.g. STRICTLY 1.19.2 and no other version
      is_strict_version = true;
    } else if delimiter_loc.unwrap() == 1 {
      // we assume that we will find a minimum version at the beginning
      return Err(ModVersionRangeParseError::NoMinimum);
    }

    if closing_loc.is_none() {
      // we assume that we will find a closing `]` or `)` somewhere
      return Err(ModVersionRangeParseError::Unclosed);
    }

    let delimiter_loc = delimiter_loc.unwrap_or(closing_loc.unwrap());
    let strlen = s.len();

    let ver_min = s[1..delimiter_loc].parse::<ModSemver>();
    let ver_max = if is_strict_version {
      None
    } else {
      s[delimiter_loc + 1..strlen - 1].parse::<ModSemver>().ok()
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

    assert_eq!(mod_version.to, None);

    assert_eq!(mod_version.mode, ModVersionRangeMode::GreaterThan)
  }

  #[test]
  fn version_range_satisfies_inclusive() {
    let version = "[1.2.3,4.5.6]".parse::<ModVersionRange>();
    let version_within = "4.5.6".parse::<ModSemver>();
    let version_outside = "9.3.4".parse::<ModSemver>();

    assert!(version.is_ok());
    assert!(version_within.is_ok());
    assert!(version_outside.is_ok());

    let version = version.unwrap();
    let version_within = version_within.unwrap();
    let version_outside = version_outside.unwrap();

    assert!(version.within_range(&version_within));
    assert!(!version.within_range(&version_outside));
  }

  #[test]
  fn version_range_satisfies_exclusive() {
    let version = "[1.2.3,4.5.6)".parse::<ModVersionRange>();
    let version_within = "4.5.5".parse::<ModSemver>();
    let version_outside = "4.5.6".parse::<ModSemver>();

    assert!(version.is_ok());
    assert!(version_within.is_ok());
    assert!(version_outside.is_ok());

    let version = version.unwrap();
    let version_within = version_within.unwrap();
    let version_outside = version_outside.unwrap();

    assert!(version.within_range(&version_within));
    assert!(!version.within_range(&version_outside));
  }

  #[test]
  fn version_range_satisfies_greater() {
    let version = "[1.2.3,)".parse::<ModVersionRange>();
    let version_within = "1.3.4".parse::<ModSemver>();
    let version_outside = "1.2.2".parse::<ModSemver>();

    assert!(version.is_ok());
    assert!(version_within.is_ok());
    assert!(version_outside.is_ok());

    let version = version.unwrap();
    let version_within = version_within.unwrap();
    let version_outside = version_outside.unwrap();

    assert!(version.within_range(&version_within));
    assert!(!version.within_range(&version_outside));
  }

  #[test]
  fn mod_sem_version_compare() {
    let mod_a_version = "1.2.3".parse::<ModSemver>();
    let mod_b_version = "1.2.4".parse::<ModSemver>();

    assert!(mod_a_version.is_ok());
    assert!(mod_b_version.is_ok());
    assert!(mod_b_version.unwrap() > mod_a_version.unwrap());
  }

  #[test]
  fn test_manifest() {
    let raw = r#"modLoader="lowcodefml"
loaderVersion="[1,)"
license="Stardust Labs License"
issueTrackerURL="https://github.com/Stardust-Labs-MC/Terralith/issues"

[[mods]]
    modId="terralith"
    version="2.5.4"
    displayName="Terralith"
    displayURL="https://www.stardustlabs.net/"
    logoFile="pack.png"
    authors="Stardust Labs"
    credits="Starmute: Owner. catter1: Maintainer. TheKingWhale: Builder. Apollounknowndev: Maintainer"
    description="Explore almost 100 new biomes consisting of both realism and light fantasy, using just Vanilla blocks. Complete with several immersive structures to compliment the overhauled terrain."

[[dependencies.terralith]]
    modId="forge"
    mandatory=false
    versionRange="[46,)"
    ordering="NONE"
    side="BOTH"
[[dependencies.terralith]]
    modId="neoforge"
    mandatory=false
    versionRange="[20.2,)"
    ordering="NONE"
    side="BOTH"
[[dependencies.terralith]]
    modId="minecraft"
    mandatory=true
    versionRange="[1.20,1.21)"
    ordering="NONE"
    side="BOTH""#;
    let parsed = toml::from_str(raw);

    assert!(parsed.is_ok());

    let mod_manifest: ForgeMod = parsed.unwrap();

    assert!(mod_manifest.mods.first().unwrap().homepage_url.is_some());
    assert!(mod_manifest.dependencies.is_some_and(|deps| {
      if deps.is_empty() || !deps.contains_key("terralith") {
        return false;
      }

      let terralith_deps = deps
        .get("terralith")
        .expect("expected `terralith` to exist");

      let minecraft_exists = terralith_deps
        .iter()
        .any(|dep| dep.id == "minecraft".into());
      let neoforge_exists = terralith_deps.iter().any(|dep| dep.id == "neoforge".into());
      let forge_exists = terralith_deps.iter().any(|dep| dep.id == "forge".into());

      minecraft_exists && neoforge_exists && forge_exists
    }))
  }
}
