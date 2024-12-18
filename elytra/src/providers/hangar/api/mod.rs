pub mod project;
pub mod version;

pub use project::search_project;
pub use version::{get_download_link, get_version, get_versions};

const HANGAR_ENDPOINT: &str = "https://hangar.papermc.io";
