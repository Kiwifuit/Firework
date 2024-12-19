use crate::errors::*;

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use crate::providers::modrinth::{
  get_versions, resolve_dependencies, search_project, Client, Facet, IndexBy, Loader,
  ModrinthProjectVersion, ProjectQueryBuilder, ProjectType, VersionQueryBuilder,
};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use toml::{from_str, to_string};

#[derive(Debug, Deserialize, Serialize)]
pub struct ModpackMetadata {
  pub name: String,
  pub version: String,
  pub description: String,
  pub author: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoaderMetadata {
  pub loader: Loader,
  pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ElytraManifest {
  pub modpack: ModpackMetadata,
  pub loader: LoaderMetadata,
  pub dependencies: HashMap<String, String>,
}

impl ElytraManifest {
  pub fn from_path<P: AsRef<Path>>(file: P) -> Result<Self, ManifestError> {
    let mut file = OpenOptions::new().read(true).open(file)?;
    let mut buf = String::new();

    file.read_to_string(&mut buf)?;
    Self::from_str(buf)
  }

  fn from_str<S: ToString>(contents: S) -> Result<Self, ManifestError> {
    let manifest = from_str(&(contents.to_string()))?;

    Ok(manifest)
  }

  pub fn save_to<P: AsRef<Path>>(self, file: P) -> Result<(), ManifestError> {
    let manifest_str = to_string(&self)?;
    let mut file = OpenOptions::new().write(true).truncate(true).open(file)?;

    file.write_all(manifest_str.as_bytes())?;

    Ok(())
  }

  pub async fn fetch_dependencies(
    &mut self,
    client: &Client,
  ) -> Result<Vec<ModrinthProjectVersion>, ManifestError> {
    info!("{} mod(s) to fetch", self.dependencies.len() - 1);
    let mut mods = vec![];
    // TODO: THIS CAN KILL A PROGRAM
    let minecraft_version = std::rc::Rc::new(self.dependencies.remove("minecraft").unwrap());

    for (name, version) in self.dependencies.iter() {
      debug!("Resolving {:?} v{}", name, version);

      let query = ProjectQueryBuilder::new()
        .query(name)
        .index_by(IndexBy::Relevance)
        .facets(vec![
          vec![Facet::Loader(self.loader.loader.clone())],
          vec![Facet::Version(minecraft_version.clone().to_string())],
          vec![Facet::ProjectType(ProjectType::Mod)],
        ])
        .build();

      let query_response = search_project(client, &query).await?;
      let projects = query_response.hits;
      let project = projects
        .iter()
        .find(|p| &p.slug.to_string() == name)
        .ok_or(ManifestError::NoHits(name.to_owned()))?;

      let version_query = VersionQueryBuilder::new()
        .versions(vec![&minecraft_version])
        .loaders(vec![self.loader.loader.clone()])
        .build();

      let mut version = get_versions(client, project, &version_query)
        .await
        .inspect_err(|e| error!("Error while fetching versions: {}", e))?
        .into_iter()
        .find(|mod_version| {
          &mod_version.id.to_string() == version
            || &mod_version.version_number.to_string() == version
            || &mod_version.name.to_string() == version
        })
        .ok_or(ManifestError::NoHits(name.to_owned()))?;

      debug!("Resolved {} to v{}", name, version.version_number);

      let resp = resolve_dependencies(client, &mut version, &version_query, |versions| {
        versions.into_iter().next().unwrap()
      })
      .await;

      match resp {
        Err(crate::errors::APIError::NoDependencies) | Ok(()) => {
          info!("Mod was resolved successfully");
          mods.push(version);
        }
        Err(e) => {
          error!("Mod was not resolved successfully: {}", e);
        }
      }
    }

    Ok(mods)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn manifest_read() {
    let manifest_raw = r#"[modpack]
name = "Elytra Sample Modpack"
version = "1.0.0"
description = "A sample modpack for Elytra"
author = "Kiwifuit <mahkiwi123@gmail.com>"

[loader]
loader = "forge"
version = "47.3.12"

[dependencies]
minecraft = "1.20.1"
entityculling = "1.7.1"
ferrite-core = "6.0.1"
xaeros-minimap = "24.6.1"
immediatelyfast = "1.3.3+1.20.4-forge"
appleskin = "2.5.1+mc1.20.1"
"#;
    let manifest = ElytraManifest::from_str(manifest_raw);

    assert!(manifest.is_ok_and(|m| !m.dependencies.is_empty()));
  }

  #[tokio::test]
  async fn manifest_fetch() {
    env_logger::init();

    let manifest_raw = r#"[modpack]
name = "Elytra Sample Modpack"
version = "1.0.0"
description = "A sample modpack for Elytra"
author = "Kiwifuit <mahkiwi123@gmail.com>"

[loader]
loader = "forge"
version = "47.3.12"

[dependencies]
minecraft = "1.20.1"
entityculling = "1.7.1"
ferrite-core = "6.0.1"
xaeros-minimap = "24.6.1"
immediatelyfast = "1.3.3+1.20.4-forge"
appleskin = "2.5.1+mc1.20.1""#;

    let client = crate::providers::modrinth::get_client()
      .await
      .expect("expected client to be constructed");
    let manifest = ElytraManifest::from_str(manifest_raw);

    assert!(manifest.as_ref().is_ok_and(|m| !m.dependencies.is_empty()));

    let deps = manifest.unwrap().fetch_dependencies(&client).await;

    assert!(deps.is_ok());
    info!("dependency graph:\n{:#?}", deps.unwrap());
  }
}
