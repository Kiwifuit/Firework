#![expect(clippy::ptr_arg)]
#[cfg(feature = "types")]
use serde::{Deserialize, Serialize, Serializer};
use std::rc::Rc;

#[cfg(feature = "types")]
pub mod project;
#[cfg(feature = "types")]
pub mod query;
#[cfg(feature = "types")]
pub mod result;
#[cfg(feature = "types")]
pub mod version;

#[cfg(feature = "types")]
pub use query::{Facet, FacetOp};

#[cfg(feature = "types")]
pub(crate) trait ModrinthProjectMeta {
  type Id;

  fn project_id(&self) -> Option<Self::Id>;
  fn version_id(&self) -> Option<Self::Id> {
    None
  }
}

#[cfg(feature = "types")]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
/// The license of a project. Can be
/// a `Single` or `Detailed` license
pub enum License {
  /// License Type
  Single(Rc<str>),
  Detailed {
    /// License ID
    id: Rc<str>,
    /// License pretty name
    name: Rc<str>,
    /// URL where the license can be found
    url: Option<Rc<str>>,
  },
}

#[cfg(feature = "types")]
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ModRequirement {
  Optional,
  Required,
  Unsupported,
  Unknown,
}

#[cfg(feature = "types")]
#[derive(Debug, Serialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
/// The mode to index by
pub enum IndexBy {
  #[default]
  Relevance,
  Downloads,
  Follows,
  Newest,
  Updated,
}

#[cfg(feature = "types")]
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
/// The loaders Modrinth supports
pub enum Loader {
  Bukkit,
  Bungeecord,
  Canvas,
  Datapack,
  Fabric,
  Folia,
  Forge,
  Iris,
  Liteloader,
  Minecraft,
  Modloader,
  Neoforge,
  Optifine,
  Purpur,
  Quilt,
  Rift,
  Spigot,
  Sponge,
  Vanilla,
  Velocity,
  Waterfall,
}

impl ToString for Loader {
  fn to_string(&self) -> String {
    match self {
      Self::Bukkit => "bukkit",
      Self::Bungeecord => "bungeecord",
      Self::Canvas => "canvas",
      Self::Datapack => "datapack",
      Self::Fabric => "fabric",
      Self::Folia => "folia",
      Self::Forge => "forge",
      Self::Iris => "iris",
      Self::Liteloader => "liteloader",
      Self::Minecraft => "minecraft",
      Self::Modloader => "modloader",
      Self::Neoforge => "neoforge",
      Self::Optifine => "optifine",
      Self::Purpur => "purpur",
      Self::Quilt => "quilt",
      Self::Rift => "rift",
      Self::Spigot => "spigot",
      Self::Sponge => "sponge",
      Self::Vanilla => "vanilla",
      Self::Velocity => "velocity",
      Self::Waterfall => "waterfall",
    }
    .to_string()
  }
}

impl Serialize for Loader {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.to_string())
  }
}

#[cfg(feature = "types")]
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
  Mod,
  Modpack,
  Resourcepack,
  Shader,
}

impl ToString for ProjectType {
  fn to_string(&self) -> String {
    match self {
      Self::Mod => "mod",
      Self::Modpack => "modpack",
      Self::Resourcepack => "resourcepack",
      Self::Shader => "shader",
    }
    .to_string()
  }
}

impl Serialize for ProjectType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.to_string())
  }
}

#[cfg(feature = "types")]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Gallery {
  Single(Rc<str>),
  Multiple(Vec<Rc<str>>),
}

pub(crate) fn serialize_vec_urlencoded<S, T>(vec: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
  T: Serialize + ToString,
{
  let vec_str = serialize_vec(vec);

  serializer.serialize_str(&vec_str)
}

pub(crate) fn serialize_vec_nested<S, T>(
  vec: &Vec<Vec<T>>,
  serializer: S,
) -> Result<S::Ok, S::Error>
where
  S: Serializer,
  T: Serialize + ToString,
{
  let vec_vec_str = format!(
    "[{}]",
    vec
      .iter()
      .map(serialize_vec)
      .collect::<Vec<String>>()
      .join(", ")
  );

  serializer.serialize_str(&vec_vec_str)
}

fn serialize_vec<T>(vec: &Vec<T>) -> String
where
  T: ToString,
{
  format!(
    "[{}]",
    vec
      .iter()
      .map(|a| format!("{:?}", a.to_string()))
      .collect::<Vec<_>>()
      .join(", ")
  )
}

#[expect(clippy::trivially_copy_pass_by_ref)]
pub(in crate::types) fn is_zero(num: &u8) -> bool {
  *num == 0
}
