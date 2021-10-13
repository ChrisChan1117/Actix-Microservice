use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
struct ErrorResponse {
  code: u16,
  error: String,
  message: String,
}

#[derive(Error, Debug)]
pub enum CustomError {
  #[error("Requested user was not found")]
  NotFound,
  #[error("You ared forbidden to access this resource.")]
  Forbidden,
  #[error("Unknown Internal Error")]
  Unknown,
  #[error("No Cookie was found")]
  NoCookie,
  #[error("Wrong Token")]
  WrongToken,
  #[error("No user found")]
  NoUserFound,
}

impl CustomError {
  pub fn name(&self) -> String {
    match self {
      Self::NotFound => "NotFound".to_string(),
      Self::Forbidden => "Forbidden".to_string(),
      Self::Unknown => "Unknown".to_string(),
      Self::NoCookie => "NoCookie".to_string(),
      Self::WrongToken => "WrongToken".to_string(),
      Self::NoUserFound => "NoUserFound".to_string(),
    }
  }
}
impl ResponseError for CustomError {
  fn status_code(&self) -> StatusCode {
    match *self {
      Self::NotFound => StatusCode::NOT_FOUND,
      Self::Forbidden => StatusCode::FORBIDDEN,
      Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
      Self::NoCookie => StatusCode::FORBIDDEN,
      Self::WrongToken => StatusCode::FORBIDDEN,
      Self::NoUserFound => StatusCode::FORBIDDEN,
    }
  }

  fn error_response(&self) -> HttpResponse {
    let status_code = self.status_code();
    let error_response = ErrorResponse {
      code: status_code.as_u16(),
      message: self.to_string(),
      error: self.name(),
    };
    HttpResponse::build(status_code).json(error_response)
  }
}

pub fn map_io_error(e: std::io::Error) -> CustomError {
  match e.kind() {
    std::io::ErrorKind::NotFound => CustomError::NotFound,
    std::io::ErrorKind::PermissionDenied => CustomError::Forbidden,
    _ => CustomError::Unknown,
  }
}
