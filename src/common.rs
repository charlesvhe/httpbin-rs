use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: SqlitePool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Pagination {
    pub offset: i32,
    pub limit: i32,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 10,
        }
    }
}

pub struct CommonError<T: Serialize> {
    pub err: StatusCode,
    pub data: Option<T>,
}

impl<T: Serialize> IntoResponse for CommonError<T> {
    fn into_response(self) -> Response {
        if let Some(d) = self.data {
            (self.err, Json(d)).into_response()
        } else {
            self.err.into_response()
        }
    }
}

impl From<sqlx::Error> for CommonError<String> {
    fn from(e: sqlx::Error) -> Self {
        CommonError {
            err: StatusCode::INTERNAL_SERVER_ERROR,
            data: Some(e.to_string()),
        }
    }
}
