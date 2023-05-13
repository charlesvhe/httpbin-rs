use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: DatabaseConnection,
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

impl From<sea_orm::DbErr> for CommonError<String> {
    fn from(e: sea_orm::DbErr) -> Self {
        CommonError {
            err: StatusCode::INTERNAL_SERVER_ERROR,
            data: Some(e.to_string()),
        }
    }
}
