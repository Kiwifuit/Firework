use std::{
  sync::{mpsc::channel, Arc},
  time::Duration,
};

use crate::{
  state::DMSState,
  types::{ServerBuildParams, WorkerMessage},
};
use axum::{extract::State, Json};
use log::{debug, error, info};

#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn get_servers(
  State(state): State<Arc<DMSState>>,
) -> Json<Vec<crate::types::MinecraftServer>> {
  info!("Querying servers");

  let resp = Vec::<crate::types::MinecraftServer>::new();

  resp.into()
}

#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn new_server(State(state): State<Arc<DMSState>>, Json(server): Json<ServerBuildParams>) {
  info!("Got build params:\n{:#?}", server);

  state.dispatch_job(WorkerMessage::Build(server)).await;

  //   state.dispatcher.send(server);
  //   let mut timeout_attempts = 0;
  //   let server_build = denji::MinecraftServer::new(
  //     server.server.parse::<denji::ServerSoftware>().unwrap(),
  //     &server.server_version,
  //     &server.server_version,
  //     "servers/",
  //   );

  //   let (tx, rx) = channel();
  //   let build_thread = spawn(async move {
  //     info!("Starting builder...");
  //     server_build.build_server(tx).await
  //   });

  //   info!("Awaiting data");
  //   loop {
  //     match rx.recv_timeout(CHANNEL_TIMEOUT) {
  //       Ok(line) => info!("{}", line),
  //       Err(std::sync::mpsc::RecvTimeoutError::Timeout)
  //         if timeout_attempts <= TIMEOUT_ATTEMPT_LIMIT =>
  //       {
  //         timeout_attempts += 1;
  //         error!("Timeout (occurred {} time(s) already)!", timeout_attempts);
  //       }

  //       // The timeout here should only be triggered on the (n + 1)
  //       // timeout, where n is the upper limit of attempts allowed
  //       Err(std::sync::mpsc::RecvTimeoutError::Disconnected)
  //       | Err(std::sync::mpsc::RecvTimeoutError::Timeout) => break,
  //     }
  //   }

  //   debug!("Closing");
  //   let result = build_thread
  //     .await
  //     .context("while trying to finish installer")
  //     .context("while trying to install server");

  //   match result {
  //     Ok(Ok(())) => info!("Build successful"),
  //     Ok(Err(build_err)) => error!("Error while trying to build server: {}", build_err),
  //     Err(build_panic) => {
  //       if let Some(reason) = build_panic.downcast_ref::<String>() {
  //         error!("Build thread panicked: {}", reason)
  //       } else {
  //         error!("Build thread panicked without a reason!")
  //       }
  //     }
  //   }
}
