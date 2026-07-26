pub mod fabric;
pub mod forge;

pub use fabric::FabricMod;
pub use forge::ForgeMod;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DependencyVersioningError {
    #[error("Unable to determine version matching mode")]
    CantDetermineMode,
}
