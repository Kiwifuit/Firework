use super::APIError;
use crate::{
  types::{
    query::VersionQuery,
    version::{ResolvedVersionDependency, VersionDependency},
  },
  version::{get_version, get_versions},
  ModrinthProjectVersion,
};
use log::{debug, info, warn};
use reqwest::Client;

/// Recursively resolves dependencies, using `resolver` as a helper function
/// to decide how the `resolve_dependencies` picks a version among a list.
// ## Usage
/// ```
/// use modrinth::{resolve_dependencies, get_versions, get_project, get_client, search_project, IndexBy, Loader, ProjectQueryBuilder, VersionQueryBuilder};
///
/// #[tokio::main]
/// async fn main() {
///     let client = get_client().await.unwrap();
///
///     let query = ProjectQueryBuilder::new()
///         .query("BotaniaCombat")
///         .limit(1)
///         .index_by(IndexBy::Relevance)
///         .build();
///
///     let res = search_project(&client, &query).await.unwrap();
///     let project = res.hits.first().unwrap();
///
///     let v_query = VersionQueryBuilder::new()
///         .featured(true)
///         .versions(vec!["1.20.1"])
///         .loaders(vec![Loader::Fabric])
///         .build();
///
///     let mut versions = get_versions(&client, &project, &v_query).await.unwrap();
///     let version = versions.get_mut(0).unwrap();
///
///     let _err = resolve_dependencies(&client, version, &v_query, |versions| {
///         versions.into_iter().next().unwrap() // always select the first version listed
///     })
///     .await;
///
///     assert!(version
///         .dependencies
///         .as_ref()
///         .unwrap()
///         .iter()
///         .all(|dep| dep.is_resolved()));
/// }
/// ```
pub async fn resolve_dependencies<F>(
  client: &Client,
  project: &mut ModrinthProjectVersion,
  version_params: &VersionQuery,
  resolver: F,
) -> Result<(), APIError>
where
  F: Fn(Vec<ModrinthProjectVersion>) -> ModrinthProjectVersion + Copy,
{
  if project.dependencies.is_none() || project.dependencies.as_ref().is_some_and(|v| v.is_empty()) {
    return Err(APIError::NoDependencies);
  }

  info!("Resolving dependencies for mod {}", project.name);
  for dependency in project.dependencies.as_mut().unwrap().iter_mut() {
    let unresolved_dependency = match dependency {
      VersionDependency::Resolved(ver) => {
        Err(APIError::ResolvedDependency(ver.dependency.name.clone()))
      }
      VersionDependency::Unresolved(ver) => Ok(ver),
    }?;

    if unresolved_dependency.version_id.is_none() && unresolved_dependency.project_id.is_none() {
      return Err(APIError::UnresolvableDependency);
    }

    let mut resolved_version = if unresolved_dependency.version_id.is_some() {
      get_version(client, unresolved_dependency).await?
    } else {
      warn!(
        "No version ID supplied for project {:?}",
        unresolved_dependency.project_id.as_ref().unwrap()
      );
      let version_list = get_versions(client, unresolved_dependency, version_params).await?;

      if version_list.len() == 1 {
        debug!("Only 1 version found, returning that");
        version_list.into_iter().next().unwrap()
      } else {
        debug!(
          "{} versions found with the matching criterion",
          version_list.len()
        );
        resolver(version_list)
      }
    };

    if resolved_version
      .dependencies
      .as_ref()
      .is_some_and(|deps| !deps.is_empty())
    {
      debug!(
        "Resolving dependency {}'s dependencies",
        resolved_version.name
      );
      Box::pin(resolve_dependencies(
        client,
        &mut resolved_version,
        version_params,
        resolver,
      ))
      .await?;
    }

    info!(
      "Mod with ID {} resolved to {}",
      resolved_version.id, resolved_version.name
    );
    *dependency = VersionDependency::Resolved(ResolvedVersionDependency {
      dependency: resolved_version,
      dependency_type: unresolved_dependency.dependency_type.clone(),
    });
  }

  info!("All dependnecies resolved!");
  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::{
    get_client, search_project, IndexBy, Loader, ProjectQueryBuilder, VersionQueryBuilder,
  };

  #[tokio::test]
  async fn check_dep_resolution() {
    let client = get_client().await.unwrap();

    let query = ProjectQueryBuilder::new()
      .query("BotaniaCombat")
      .limit(1)
      .index_by(IndexBy::Relevance)
      .build();

    let res = search_project(&client, &query).await.unwrap();
    let project = res.hits.first().unwrap();

    let v_query = VersionQueryBuilder::new()
      .featured(true)
      .versions(vec!["1.20.1"])
      .loaders(vec![Loader::Fabric])
      .build();

    let mut versions = get_versions(&client, &project, &v_query).await.unwrap();
    let version = versions.get_mut(0).unwrap();

    let _err = resolve_dependencies(&client, version, &v_query, |versions| {
      versions.into_iter().next().unwrap()
    })
    .await;

    assert!(version
      .dependencies
      .as_ref()
      .unwrap()
      .iter()
      .all(|dep| dep.is_resolved()));
  }
}
