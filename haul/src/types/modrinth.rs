use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use super::ModpackProviderMetadata;
use serde::{Deserialize, Deserializer};
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModrinthModpack {
    pub game: Rc<str>,
    pub version_id: Rc<str>,
    pub name: Rc<str>,
    pub summary: Option<Rc<str>>,
    pub files: Rc<[ModrinthModpackFiles]>,
    #[serde(deserialize_with = "deserialize_deps")]
    pub dependencies: Rc<[ModrinthModpackDependency]>,
}

impl ModpackProviderMetadata for ModrinthModpack {
    type Out = Rc<str>;

    fn overrides_dir(&self) -> Self::Out {
        "overrides".into()
    }

    fn modpack_name(&self) -> Self::Out {
        self.name.clone()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModrinthModpackFiles {
    pub path: Box<Path>,
    pub hashes: ModpackFileHashes,
    pub env: Option<ModpackEnv>,
    pub downloads: Vec<Rc<str>>,
    pub file_size: usize,
}

#[derive(Debug, Deserialize)]
pub struct ModpackFileHashes {
    pub sha1: Rc<str>,
    pub sha512: Rc<str>,
}

#[derive(Debug, Deserialize)]
pub struct ModpackEnv {
    pub server: Rc<str>,
    pub client: Rc<str>,
}

#[derive(Debug, Deserialize)]
pub struct ModrinthModpackDependency {
    pub dependency: Rc<str>,
    pub version: Rc<str>,
}

fn deserialize_deps<'de, D>(deserializer: D) -> Result<Rc<[ModrinthModpackDependency]>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_map: HashMap<Rc<str>, Rc<str>> = HashMap::deserialize(deserializer)?;
    let deps = raw_map
        .into_iter()
        .map(|(dependency, version)| ModrinthModpackDependency {
            dependency,
            version,
        })
        .collect();

    Ok(deps)
}
