use super::ModpackProviderMetadata;
use serde::Deserialize;
use std::rc::Rc;
#[derive(Debug, Deserialize)]
pub struct ForgeModpack {
    pub minecraft: ModpackLoaderMeta,
    pub name: Rc<str>,
    pub version: Rc<str>,
    pub author: Rc<str>,
    pub files: Rc<[ModpackFiles]>,
    overrides: Rc<str>,
}

impl ModpackProviderMetadata for ForgeModpack {
    type Out = Rc<str>;

    fn overrides_dir(&self) -> Self::Out {
        self.overrides.clone()
    }

    fn modpack_name(&self) -> Self::Out {
        self.name.clone()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModpackLoaderMeta {
    pub version: Rc<str>,
    pub mod_loaders: Vec<ModpackLoaderVersion>,
}

#[derive(Debug, Deserialize)]
pub struct ModpackLoaderVersion {
    pub id: Rc<str>,
    pub primary: bool,
}

#[derive(Debug, Deserialize)]
pub struct ModpackFiles {
    #[serde(rename = "projectID")]
    pub project_id: u32,
    #[serde(rename = "fileID")]
    pub file_id: u32,
    pub required: bool,
}
