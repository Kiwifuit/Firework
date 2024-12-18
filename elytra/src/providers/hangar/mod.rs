#[cfg(feature = "api")]
pub mod api;
#[cfg(feature = "types")]
pub mod types;

#[cfg(feature = "api")]
pub use api::{get_download_link, get_version, get_versions, search_project};
#[cfg(feature = "types")]
pub use types::query::{GenericPagination, SearchQueryBuilder, VersionQueryBuilder};
#[cfg(feature = "types")]
pub use types::{HangarProjects, HangarVersions, HangarVisibility};
