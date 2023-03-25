use axum::response::{IntoResponse, Json, Response};
use serde::Serialize;

pub type Result<T> = std::result::Result<T, ErrorCode>;

pub enum ErrorCode {
    GenerateToken,
    TokenParse,
}

impl IntoResponse for ErrorCode {
    fn into_response(self) -> Response {
        Json(ErrorResponse {
            code: 1,
            msg: Some(""),
        })
        .into_response()
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponse<'a> {
    code: i64,
    msg: Option<&'a str>,
}
