use axum::{response::{IntoResponse, Response}, http::StatusCode};

pub struct AppError(pub anyhow::Error, pub StatusCode);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.1, self.0.to_string()).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error> 
{
    fn from(err: E) -> Self {
        Self(err.into(), StatusCode::INTERNAL_SERVER_ERROR)
    }
}