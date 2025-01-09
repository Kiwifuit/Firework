#[cfg(feature = "all")]
pub mod repository;
#[cfg(any(feature = "types", feature = "type-maven-artifact"))]
pub mod types;

#[cfg(feature = "all")]
pub use repository::*;
#[cfg(all(feature = "types", feature = "type-maven-artifact"))]
pub use types::MavenArtifactBuilder;

#[cfg(all(not(feature = "types"), feature = "type-maven-artifact"))]
pub use types::MavenArtifact;
