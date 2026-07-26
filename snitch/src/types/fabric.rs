use serde::Deserialize;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct FabricMod {
    #[serde(rename = "schemaVersion")]
    _schema_version: u8,
    #[serde(rename = "entrypoints")]
    _entrypoints: Option<HashMap<Rc<str>, Vec<Rc<str>>>>,
    #[serde(rename = "accessWidener")]
    _access_widener: Option<Rc<str>>,
    #[serde(rename = "jars")]
    _jars: Option<Vec<HashMap<Rc<str>, Rc<str>>>>,

    #[serde(rename = "id")]
    pub mod_id: Rc<str>,
    pub icon: Rc<str>,
    #[serde(rename = "version")]
    pub mod_version: Rc<str>,
    pub name: Option<Rc<str>>,
    pub description: Option<Rc<str>>,
    pub authors: Option<Vec<Rc<str>>>,
    pub contributors: Option<Vec<Rc<str>>>,
    pub contact: Option<FabricModContact>,
    pub license: Option<Rc<str>>,
    #[serde(rename = "depends")]
    pub dependencies: Option<HashMap<Rc<str>, FabricDependencyVersion>>,
    pub recommends: Option<HashMap<Rc<str>, FabricDependencyVersion>>,
    pub conflicts: Option<HashMap<Rc<str>, FabricDependencyVersion>>,
    pub breaks: Option<HashMap<Rc<str>, FabricDependencyVersion>>,
}

#[derive(Debug, Deserialize)]
pub struct FabricModContact {
    pub homepage: Option<Rc<str>>,
    pub issues: Option<Rc<str>>,
    pub sources: Option<Rc<str>>,
    pub email: Option<Rc<str>>,
    pub irc: Option<Rc<str>>,
}

#[derive(Debug, Deserialize)]
pub enum FabricDependencyVersionMode {
    Any,
    ExactMatch,
    SameMinor,
    SameMajor,
    GreaterThan,
    LesserThan,
    GreaterThanEqual,
    LesserThanEqual,
}

#[derive(Debug)]
pub struct FabricDependencyVersion {
    pub mode: FabricDependencyVersionMode,
    pub version: Rc<str>,
}

impl<'de> Deserialize<'de> for FabricDependencyVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mod_data = String::deserialize(deserializer)?;
        mod_data.parse().map_err(serde::de::Error::custom)
    }
}

impl FabricDependencyVersion {
    fn check_equals(s: &str) -> bool {
        // For some reason, there is a chance for
        // a "*" to end up in here, so we gotta check
        // for out-of-bounds edge cases and immidiately
        // return false if we can't even begin to
        // check
        if s.len() < 2 {
            false
        } else {
            s.chars().nth(1).unwrap() == '='
        }
    }
}

impl FromStr for FabricDependencyVersion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        #[allow(clippy::wildcard_in_or_patterns)]
        let mode = match s.chars().next().unwrap() {
            any_char if any_char.is_numeric() => FabricDependencyVersionMode::ExactMatch,
            '>' if Self::check_equals(s) => FabricDependencyVersionMode::GreaterThanEqual,
            '<' if Self::check_equals(s) => FabricDependencyVersionMode::LesserThanEqual,
            '>' => FabricDependencyVersionMode::GreaterThan,
            '<' => FabricDependencyVersionMode::LesserThan,
            '^' => FabricDependencyVersionMode::SameMajor,
            '~' => FabricDependencyVersionMode::SameMinor,
            '*' | _ => FabricDependencyVersionMode::Any,
        };

        let version_start = if Self::check_equals(s) { 2 } else { 1 };
        let version = Rc::from(s[version_start..].to_string().into_boxed_str());

        Ok(Self { mode, version })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unzip::grab_meta_file;
    use serde_json::from_str;
    use std::fs::read_dir;

    #[test]
    fn mod_manifest() {
        for file in read_dir("samples/fabric/").unwrap() {
            let file = file.unwrap();

            if file.file_type().unwrap().is_dir() {
                continue;
            }

            let mod_meta = from_str::<FabricMod>(
                &grab_meta_file(file.path())
                    .expect("expected meta file to be grabbed")
                    .raw,
            );

            assert!(mod_meta.is_ok());
        }
    }
}
