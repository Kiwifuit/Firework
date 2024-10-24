use std::vec;

use anyhow::Context;
use dotenv::dotenv;
use log::info;
use serde_qs::to_string;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv().context("while loading dotenv")?;
  env_logger::init();
  info!("Hello world!");

  let curse = curseforge::CurseForge::new_from_env().context("while building client")?;
  let query = curseforge::types::query::ModQueryBuilder::default()
    .mod_name("Graves")
    .categories(vec![420, 424, 421, 425])
    .game_versions(vec!["1.12.2".to_string()]);

  dbg!(to_string(&query));
  curse.get_mod(&query).await;

  Ok(())
}
