use axum::{
    extract::FromRequest,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: String,
    pub data: T,
}

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct AppJson<T>(pub T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}
