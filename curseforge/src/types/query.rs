// use serde::{ser::SerializeSeq, Serialize};

// use super::project::CurseMod;

// #[derive(Debug)]
// pub struct CurseMods {
//   mods: Vec<CurseMod>,
// }

// impl Serialize for CurseMods {
//   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//   where
//     S: serde::Serializer,
//   {
//     let mut seq = serializer.serialize_seq(Some(self.mods.len()))?;

//     for mod_data in &self.mods {
//       seq.serialize_element(&mod_data.id)?;
//     }

//     seq.end()
//   }
// }

// impl From<Vec<CurseMod>> for CurseMods {
//   fn from(value: Vec<CurseMod>) -> Self {
//     Self { mods: value }
//   }
// }

// Idea: Might want to add a "growable"
// API. Where you start with an empty
// CurseMods struct and iteratively
// fill it or something
//
// Something like this:
// impl CurseMods {
//   pub fn new() -> Self {
//     Self {
//       mods: vec![]
//     }
//   }

//   pub fn add_mod(&mut self, new_mod: CurseMod) {
//     self.mods.push(new_mod);
//   }
// }

use serde::{ser::SerializeSeq, Serialize, Serializer};

const CURSE_MINECRAFT_ID: u16 = 432;

#[derive(Debug, Default, Clone, Copy)]
#[repr(u8)]
pub enum SortBy {
  Featured = 1,
  #[default]
  Popularity = 2,
  LastUpdated = 3,
  Name = 4,
  Author = 5,
  TotalDownloads = 6,
  Category = 7,
  GameVersion = 8,
  EarlyAccess = 9,
  FeaturedReleased = 10,
  ReleasedDate = 11,
  Rating = 12,
}

impl Serialize for SortBy {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_u8(*self as u8)
  }
}

#[derive(Debug, Serialize, Default)]
pub enum OrderBy {
  #[serde(rename = "asc")]
  #[default]
  Ascending,
  #[serde(rename = "desc")]
  Descending,
}

#[derive(Debug, Default, Serialize, Clone, Copy)]
#[repr(u8)]
pub enum ModLoader {
  #[default]
  Any = 0,
  Forge = 1,
  Cauldron = 2,
  LiteLoader = 3,
  Fabric = 4,
  Quilt = 5,
  NeoForge = 6,
}

impl ToString for ModLoader {
  fn to_string(&self) -> String {
    (*self as u8).to_string()
  }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModQueryBuilder {
  pub(crate) game_id: u16,
  #[serde(serialize_with = "serialize_vec")]
  pub(crate) categories: Vec<usize>,
  #[serde(serialize_with = "serialize_vec")]
  pub(crate) game_versions: Vec<String>,
  pub(crate) search_filter: String,
  pub(crate) sort_field: SortBy,
  pub(crate) sort_order: OrderBy,
  #[serde(serialize_with = "serialize_vec")]
  pub(crate) mod_loader_types: Vec<ModLoader>,
  pub(crate) index: usize,
}

impl Default for ModQueryBuilder {
  fn default() -> Self {
    Self {
      game_id: CURSE_MINECRAFT_ID,
      categories: Default::default(),
      game_versions: Default::default(),
      search_filter: Default::default(),
      sort_field: Default::default(),
      sort_order: Default::default(),
      mod_loader_types: Default::default(),
      index: Default::default(),
    }
  }
}

impl ModQueryBuilder {
  pub fn mod_name<S: ToString>(mut self, search: S) -> Self {
    self.search_filter = search.to_string();

    self
  }

  pub fn categories(mut self, categories: Vec<usize>) -> Self {
    self.categories = categories;

    self
  }

  pub fn game_versions(mut self, game_versions: Vec<String>) -> Self {
    self.game_versions = game_versions;

    self
  }

  pub fn sort_field(mut self, sort_field: SortBy) -> Self {
    self.sort_field = sort_field;

    self
  }

  pub fn sort_order(mut self, sort_order: OrderBy) -> Self {
    self.sort_order = sort_order;

    self
  }

  pub fn mod_loader_types(mut self, mod_loader_types: Vec<ModLoader>) -> Self {
    self.mod_loader_types = mod_loader_types;

    self
  }

  pub fn index(mut self, index: usize) -> Self {
    self.index = index;

    self
  }
}

fn serialize_vec<T, S>(vec: &[T], serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
  T: Serialize + ToString,
{
  // TODO: use serde_json here,
  // we using that thing already anyway
  let vec_str = format!(
    "[{}]",
    vec
      .iter()
      .map(|c| {
        let c_str = c.to_string();

        if c_str.parse::<usize>().is_ok() {
          c_str
        } else {
          format!("{:?}", c_str)
        }
      })
      .collect::<Vec<_>>()
      .join(",")
  );

  serializer.serialize_str(&vec_str)
}
