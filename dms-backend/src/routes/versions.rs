use axum::Json;

pub async fn get_loaders() -> Json<Vec<String>> {
  Json(
    [
      "NeoForge",
      "Forge",
      "Fabric",
      "Quilt",
      "Glowstone",
      "Arclight",
    ]
    .iter()
    .map(ToString::to_string)
    .collect::<Vec<_>>(),
  )
}
