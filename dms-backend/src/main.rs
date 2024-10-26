use std::env::var;

use anyhow::Context;
use axum::routing::get;
use axum::Router;
use http::{HeaderValue, Method};
use log::info;
use owo_colors::OwoColorize;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod logger;
mod routes;
mod types;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> anyhow::Result<()> {
  logger::init().context("while initializing logger")?;

  info!("Good morning!");
  let ip_addr = var("DMS_HOST").unwrap_or(String::from("0.0.0.0:3030"));

  let cors = CorsLayer::new()
    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    .allow_methods(vec![Method::GET, Method::POST]);

  let app = Router::new()
    .route("/servers", get(routes::get_servers))
    .layer(cors);

  let server = TcpListener::bind(&ip_addr).await.unwrap();

  info!("Listening on {}", ip_addr.green());
  axum::serve(server, app)
    .await
    .context("while running backend server")?;

  Ok(())
}
