use std::ffi::OsStr;

use libloading::{Library, Symbol};
use log::info;
use thiserror::Error;

use crate::plugin::ServerProvider;

type PluginInit = fn() -> Box<dyn ServerProvider>;

#[derive(Debug, Error)]
pub enum PluginError {
  #[error("while loading plugin: {0}")]
  Load(#[from] libloading::Error),
}

#[repr(transparent)]
pub struct Plugin {
  plugin: Box<dyn ServerProvider>,
}

impl Drop for Plugin {
  fn drop(&mut self) {
    info!(
      "Unloading plugin {} v{}",
      self.plugin.id(),
      self.plugin.version()
    );
  }
}

impl Plugin {
  pub fn load<P: AsRef<OsStr>>(path: P) -> Result<Self, PluginError> {
    let plugin = unsafe { Library::new(path) }?;
    let plugin_init: Symbol<PluginInit> = unsafe { plugin.get(b"init") }?;

    Ok(Self {
      plugin: plugin_init(),
    })
  }
}
