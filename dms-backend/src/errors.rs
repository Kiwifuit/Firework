use axum::response::IntoResponse;
use http::StatusCode;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
  #[error("I/O Error within the server: {0}")]
  Io(#[from] std::io::Error),
}

// impl Serialize for ServerError {
//   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//   where
//     S: serde::Serializer,
//   {
//     let mut err = serializer.serialize_struct("Error", 2)?;
//     let variant = match self {
//       Self::Io(_) => "IoError",
//     };
//     let message = self.to_string();

//     err.serialize_field("error", &variant)?;
//     err.serialize_field("description", &message)?;

//     err.end()
//   }
// }

impl IntoResponse for ServerError {
  fn into_response(self) -> axum::response::Response {
    let status_code = StatusCode::INTERNAL_SERVER_ERROR;
    let body = json!({
        "body": match self {
            Self::Io(_) => "I/O Error",
        },
        "description": self.to_string()
    });

    (status_code, axum::Json(body)).into_response()
  }
}

#[derive(Debug, Error)]
pub enum StateError {
  #[error("I/O Error: {0}")]
  Io(#[from] std::io::Error),

  #[error("JSON de/serialization error: {0}")]
  Json(#[from] serde_json::Error),

  #[error("DMS failed to create a data dir for itself")]
  DataDirFail,
}
