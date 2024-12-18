#[cfg(feature = "api")]
pub mod api;
#[cfg(feature = "types")]
pub mod types;

#[cfg(feature = "api")]
pub use api::*;
#[cfg(feature = "types")]
pub use types::project::ModrinthProject;
#[cfg(feature = "types")]
pub use types::query::{ProjectQueryBuilder, VersionQueryBuilder};
#[cfg(feature = "types")]
pub use types::version::ModrinthProjectVersion;
#[cfg(feature = "types")]
pub use types::{Facet, FacetOp, IndexBy, Loader, ProjectType};

pub use reqwest::Client;
