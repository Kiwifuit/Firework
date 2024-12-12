use crate::errors::*;
use std::{
  collections::HashMap,
  fs::OpenOptions,
  io::{Read, Write},
  path::Path,
};

use serde::{Deserialize, Serialize};
use toml::{from_str, to_string};

#[derive(Debug, Deserialize, Serialize)]
pub struct ModpackMetadata {
  name: String,
  version: String,
  description: String,
  author: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoaderMetadata {
  loader: String,
  version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ElytraManifest {
  modpack: ModpackMetadata,
  loader: LoaderMetadata,
  dependencies: HashMap<String, String>,
}

impl ElytraManifest {
  pub fn from_path<P: AsRef<Path>>(file: P) -> Result<Self, ManifestError> {
    let mut file = OpenOptions::new().read(true).open(file)?;
    let mut buf = String::new();

    file.read_to_string(&mut buf)?;

    let manifest = from_str(&buf)?;

    Ok(manifest)
  }

  pub fn save_to<P: AsRef<Path>>(self, file: P) -> Result<(), ManifestError> {
    let manifest_str = to_string(&self)?;
    let mut file = OpenOptions::new().write(true).truncate(true).open(file)?;

    file.write_all(manifest_str.as_bytes())?;

    Ok(())
  }
}
