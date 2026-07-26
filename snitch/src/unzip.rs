use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::rc::Rc;

use log::{debug, error, info};
use thiserror::Error;
use zip::ZipArchive;

const FORGE_META: &str = "META-INF/mods.toml";
const FABRIC_META: &str = "fabric.mod.json";

pub struct ModMeta {
    pub loader: ModLoader,
    pub raw: Rc<str>,
}

pub enum ModLoader {
    Forge,
    Fabric,
    None,
}

#[derive(Error, Debug)]
pub enum UnzipError {
    #[error("unable to open file: {0}")]
    FileOpen(#[from] std::io::Error),

    #[error("error during reading the zip file: {0}")]
    ZipRead(#[from] zip::result::ZipError),

    #[error("/META-INF/mods.toml file not found within mod")]
    MetaFileNotFound,

    #[error("temporary file was not made")]
    TempFileNotMade,

    #[error("write to temporary file was not made")]
    WriteToTempFile,
}

pub fn grab_meta_file<F: AsRef<Path>>(file: F) -> Result<ModMeta, UnzipError> {
    let zipfile = File::open(file)?;
    let mut archive = ZipArchive::new(zipfile)?;

    let (config_file, loader) = if archive.by_name(FORGE_META).is_ok() {
        info!("Modpack manifest found at {}", FORGE_META);
        (FORGE_META, ModLoader::Forge)
    } else if archive.by_name(FABRIC_META).is_ok() {
        info!("Modpack manifest found at {}", FABRIC_META);
        (FABRIC_META, ModLoader::Fabric)
    } else {
        error!("No manifest found!");
        ("", ModLoader::None)
    };

    let mut file = archive
        .by_name(config_file)
        .or(Err(UnzipError::MetaFileNotFound))?;

    info!("Reading manifest file at {}", config_file);
    let mut raw = String::new();
    let len = file.read_to_string(&mut raw)?;
    debug!("Read {} bytes to buffer", len);

    Ok(ModMeta {
        loader,
        raw: Rc::from(raw.into_boxed_str()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meta_get_forge() {
        let file = "samples/forge/architectury-6.6.92-forge.jar";
        let res = grab_meta_file(file);

        assert!(res.is_ok());
        assert!(!res.unwrap().raw.is_empty());
    }

    #[test]
    fn meta_get_fabric() {
        let file = "samples/fabric/antique-atlas-2.5.0+1.20.jar";
        let res = grab_meta_file(file);

        assert!(res.is_ok());
        assert!(!res.unwrap().raw.is_empty());
    }

    #[test]
    fn meta_readable() {
        let forge_mod = grab_meta_file("samples/forge/architectury-6.6.92-forge.jar");
        let fabric_mod = grab_meta_file("samples/fabric/antique-atlas-2.5.0+1.20.jar");

        assert!(forge_mod.is_ok());
        assert!(!forge_mod.unwrap().raw.is_empty());

        assert!(fabric_mod.is_ok());
        assert!(!fabric_mod.unwrap().raw.is_empty());
    }
}
