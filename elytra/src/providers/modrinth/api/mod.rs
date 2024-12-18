#![cfg_attr(not(feature = "api"), allow(unused_imports, dead_code))]

use std::{rc::Rc, time::Duration};

#[cfg(feature = "api")]
use log::{debug, error, info};
#[cfg(feature = "api")]
use reqwest::Client;
#[cfg(feature = "api")]
use thiserror::Error;

#[cfg(feature = "api")]
pub mod dependency;
#[cfg(feature = "api")]
pub mod project;
#[cfg(feature = "api")]
pub mod version;

#[cfg(feature = "api")]
pub use dependency::resolve_dependencies;
#[cfg(feature = "api")]
pub use project::{get_project, search_project};
#[cfg(feature = "api")]
pub use version::get_versions;

const ENDPOINT: &str = "https://api.modrinth.com";

#[cfg(feature = "api")]
#[derive(Debug, Error)]
pub enum APIError {
  #[error("http error: {0}")]
  Http(#[from] reqwest::Error),

  #[error("dependency already resolved: {0}")]
  ResolvedDependency(Rc<str>),

  #[error("provided mod has no dependencies")]
  NoDependencies,

  #[error("provided mod has unresolvable dependencies")]
  UnresolvableDependency,
}

#[cfg(feature = "api")]
/// Checks Modrinth's availability.
/// This function returns a `Client`, but it is up to
/// you to see if the ping to [Modrinth's API Endpoint](https://api.modrinth.com)
///
/// Please use `get_client`, which does the same thing as
/// this function but also checks if the request went through.
///
/// ## Errors
/// This function only fails if the client failed to build
///
/// ## Usage
/// ```
/// use modrinth::check_api;
///
/// #[tokio::main]
/// async fn main() {
///     let api_check = check_api().await;
///     assert!(api_check.is_ok());
///
///     let (labrinth_responding, _client) = api_check.unwrap();
///     assert!(labrinth_responding);
/// }
/// ```
pub async fn check_api() -> Result<(bool, Client), APIError> {
  debug!("building client");
  let client = Client::builder()
    .user_agent(format!(
      "{} using {} v{}",
      std::env::var("CARGO_BIN_NAME").unwrap_or(String::from("<unknown>")),
      env!("CARGO_PKG_NAME"),
      env!("CARGO_PKG_VERSION")
    ))
    .https_only(true)
    .timeout(Duration::from_secs(30))
    .connection_verbose(false)
    .redirect(reqwest::redirect::Policy::none())
    .build()?;

  debug!("vibe checking modrinth endpoint at {:?}", ENDPOINT);
  let resp = client.get(ENDPOINT).send().await;

  Ok((resp.is_ok(), client))
}

#[cfg(feature = "api")]
/// Checks if Modrinth is available and returns a `Client` if it does
///
/// ## Errors
/// Returns `None` if the underlying function `check_api` fails,
/// or if modrinth isnt available
///
/// ## Usage
/// ```
/// use modrinth::get_client;
///
/// #[tokio::main]
/// async fn main() {
///     let client = get_client().await.unwrap();
/// }
/// ```
pub async fn get_client() -> Option<Client> {
  info!("Checking api");
  let api_check = check_api().await;

  if let Err(api_err) = api_check {
    error!(
      "Error while testing Modrinth api: {:}. Are you sure you are connected to the internet?",
      api_err
    );
    return None;
  }
  let (_labrinth_responding, client) = api_check.unwrap();

  Some(client)
}

#[cfg(test)]
#[cfg(feature = "api")]
mod tests {
  use super::check_api;

  #[tokio::test]
  async fn check_api_works() {
    let api_check = check_api().await;

    assert!(api_check.is_ok());
    let (labrinth_responding, _client) = api_check.unwrap();

    assert!(labrinth_responding);
  }
}
