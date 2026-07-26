use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::{absolute, Path, PathBuf};

use log::{debug, error, info};
use thiserror::Error;
use zip::ZipArchive;

use crate::types::ModpackProviderMetadata;
use crate::ModpackProvider;

const FORGE_META: &str = "manifest.json";
const MODRINTH_META: &str = "modrinth.index.json";

#[derive(Debug, Error)]
pub enum UnzipError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("zip error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("no manifest from modrinth or forge found!")]
    NoManifest,
}

pub struct ModpackMetadata {
    pub loader: ModpackProvider,
    pub raw: String,
}

pub fn get_modpack_manifest<F: AsRef<Path>>(file: &F) -> Result<ModpackMetadata, UnzipError> {
    let zipfile = File::open(file)?;
    let mut archive = ZipArchive::new(zipfile)?;

    let (manifest_file, loader) = if archive.by_name(FORGE_META).is_ok() {
        info!("Modpack manifest found at {}", FORGE_META);
        (FORGE_META, ModpackProvider::Forge)
    } else if archive.by_name(MODRINTH_META).is_ok() {
        info!("Modpack manifest found at {}", MODRINTH_META);
        (MODRINTH_META, ModpackProvider::Modrinth)
    } else {
        error!("No manifest found!");
        ("", ModpackProvider::None)
    };

    if loader == ModpackProvider::None {
        return Err(UnzipError::NoManifest);
    }

    let mut file = archive.by_name(manifest_file).expect("expected that by here, modpack provider should be either forge or modrinth, so this should not appear at all");

    info!("Reading manifest file at {}", manifest_file);
    let mut raw = String::new();
    let len = file.read_to_string(&mut raw)?;
    debug!("Read {} bytes to buffer", len);

    Ok(ModpackMetadata { loader, raw })
}

pub fn unzip_modpack_to<Fz, Fd, M>(zipfile: Fz, dir: &Fd, manifest: &M) -> Result<(), UnzipError>
where
    Fz: AsRef<Path>,
    Fd: AsRef<Path>,
    M: ModpackProviderMetadata,
    <M as ModpackProviderMetadata>::Out: ToString,
{
    let zipfile = File::open(zipfile)?;
    let mut archive = ZipArchive::new(zipfile)?;
    let overrides_dir = manifest.overrides_dir();

    info!("Extracting archive");
    for i in 0..archive.len() {
        let mut infile = archive.by_index(i)?;
        let arcfile = infile
            .enclosed_name()
            .unwrap()
            .components()
            .enumerate()
            .filter_map(|(i, comp)| if i != 0 { Some(comp) } else { None })
            .collect::<PathBuf>();

        let outpath = absolute(dir.as_ref().join(&arcfile))?;

        if !infile.name().starts_with(&overrides_dir.to_string()) {
            continue;
        }

        if infile.is_dir() {
            info!("Creating dir: {}", outpath.display());
            create_dir_all(outpath)?;
        } else {
            info!("Extracting {} to {}", infile.name(), outpath.display());
            if !outpath.parent().unwrap().exists() {
                create_dir_all(outpath.parent().unwrap())?;
            }

            let mut outfile = File::create(outpath).unwrap();
            std::io::copy(&mut infile, &mut outfile)?;
            debug!("Extracted {}!", arcfile.display());
        }
    }

    Ok(())
}
