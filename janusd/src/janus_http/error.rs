// SPDX-License-Identifier: GPL-3.0-or-later

//! HTTP error responses.

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::janus_cfg;
use crate::janus_repo;

/// An HTTP error response with a JSON body `{"error": message}`.
pub struct ApiError {
	status: StatusCode,
	message: String,
}

impl ApiError {
	/// Creates a 400 Bad Request error.
	pub fn bad_request(message: impl Into<String>) -> Self {
		ApiError {
			status: StatusCode::BAD_REQUEST,
			message: message.into(),
		}
	}

	/// Creates a 404 Not Found error.
	pub fn not_found(message: impl Into<String>) -> Self {
		ApiError {
			status: StatusCode::NOT_FOUND,
			message: message.into(),
		}
	}

	/// Creates a 500 Internal Server Error.
	pub fn internal(message: impl Into<String>) -> Self {
		ApiError {
			status: StatusCode::INTERNAL_SERVER_ERROR,
			message: message.into(),
		}
	}
}

impl IntoResponse for ApiError {
	fn into_response(self) -> Response {
		(self.status, Json(serde_json::json!({ "error": self.message }))).into_response()
	}
}

impl From<janus_cfg::Error> for ApiError {
	fn from(err: janus_cfg::Error) -> Self {
		match err {
			janus_cfg::Error::InvalidName(_) | janus_cfg::Error::InvalidValue(_) => ApiError {
				status: StatusCode::UNPROCESSABLE_ENTITY,
				message: err.to_string(),
			},
			_ => ApiError::internal("internal store error"),
		}
	}
}

impl From<janus_repo::Error> for ApiError {
	fn from(err: janus_repo::Error) -> Self {
		match err {
			janus_repo::Error::InvalidName(_) | janus_repo::Error::InvalidValue(_) => ApiError {
				status: StatusCode::UNPROCESSABLE_ENTITY,
				message: err.to_string(),
			},
			_ => ApiError::internal("internal store error"),
		}
	}
}
