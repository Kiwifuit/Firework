use std::env::var;

use anyhow::Context;
use axum::routing::{get, post};
use axum::Router;
use http::{HeaderName, HeaderValue, Method};
use log::info;
use owo_colors::OwoColorize;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod logger;
mod routes;
mod types;
mod ws;

// Route overview:
// [POST, GET]          /api/servers
// [POST]               /api/mod/search?sid=<server id>&query=<name>
// [POST]               /api/modpack/search?sid=<server id>&query=<name>
// [POST]               /api/datapack/search?sid=<server id>&query=<name>
// [POST]               /api/dashboard?sid=<server id>
// [POST, GET]          /api/options?sid=<server id>
// [POST, GET, DELETE]  /api/players/[list,add,modify,delete]
// [POST, GET]          /api/worlds/[upload,download]
//
// ws://localhost:3030/api/console
// ws://localhost:3030/api/logs

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> anyhow::Result<()> {
  logger::init().context("while initializing logger")?;

  info!("Good morning!");
  let ip_addr = var("DMS_HOST").unwrap_or(String::from("localhost:3030"));

  let cors = CorsLayer::new()
    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    .allow_methods(vec![Method::GET, Method::POST])
    .allow_headers(vec![HeaderName::from_static("content-type")]);

  let app = Router::new()
    .route("/servers", get(routes::get_servers))
    .route("/servers", post(routes::new_server))
    .layer(cors);

  let server = TcpListener::bind(&ip_addr)
    .await
    .context(format!("while binding to {:?}", ip_addr))?;

  info!("Listening on {}", ip_addr.green());
  axum::serve(server, app)
    .await
    .context("while running backend server")?;

  Ok(())
}
