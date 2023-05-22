use std::fmt::Display;

use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = core::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
  NotFound,
  LoginFail,
}

impl Display for AppError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self:?}")
  }
}

impl IntoResponse for AppError {
  fn into_response(self) -> axum::response::Response {
    println!("->> {:<15} - {self:?}", "INTO RES");
    (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLER_CLIENT_ERROR").into_response()
  }
}
