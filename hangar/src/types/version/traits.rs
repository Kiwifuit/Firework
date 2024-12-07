use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Deserializer};

pub(super) trait KeyValueType {
  type Key;
  type Value;

  fn init(key: Self::Key, value: Self::Value) -> Self;
}

pub(super) fn deserialize_kv<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
  D: Deserializer<'de>,
  T: KeyValueType,
  T::Key: Deserialize<'de> + Hash + Eq,
  T::Value: Deserialize<'de>,
{
  let raw_map: HashMap<T::Key, T::Value> = HashMap::deserialize(deserializer)?;
  let deps = raw_map
    .into_iter()
    .map(|(key, value)| T::init(key, value))
    .collect();

  Ok(deps)
}
