use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::errors::{AppError, Result};

// Region: Routes
pub fn routes() -> Router {
  Router::new().route("/api/login", post(api_login))
}
// End-Region: Routes

#[derive(Debug, Deserialize)]
struct LoginPayload {
  username: String,
  pwd: String,
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
  println!("->> {:<15} - api_login", "HANDLER");

  if payload.username != "eamon" || payload.pwd != "1234" {
    return Err(AppError::LoginFail);
  }

  let body = Json(json!({
    "result": {
      "success" : true
    }
  }));

  Ok(body)
}
