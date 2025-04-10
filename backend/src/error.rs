use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::response::{ApiResponse, AppJson};

pub enum AppError {
    UnknownError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::UnknownError(resp) => (StatusCode::INTERNAL_SERVER_ERROR, resp),
        };

        (
            status,
            AppJson(ApiResponse {
                status: status.to_string(),
                message,
                data: "".to_string(),
            }),
        )
            .into_response()
    }
}
