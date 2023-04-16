use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::QueryBuilder;

use crate::common::{AppState, CommonError, Pagination};

pub fn init_router() -> axum::Router {
    Router::new()
        .route("/ConfigMeta", axum::routing::get(get).post(post))
        .route(
            "/ConfigMeta/:id",
            axum::routing::get(get_one).put(put).delete(delete),
        )
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct ConfigMeta {
    pub id: i64,
    pub code: String,
    pub property: String,
    pub column_name: String,
    pub description: String,
    pub sort: i32,
    pub gmt_create: String,
    pub gmt_modified: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct ConfigMetaUpdate {
    pub code: Option<String>,
    pub property: Option<String>,
    pub column_name: Option<String>,
    pub description: Option<String>,
    pub sort: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct Filter {
    pub code: Option<String>,
}

pub async fn get(
    pagination: Query<Pagination>,
    filter: Query<Filter>,
    Extension(state): Extension<AppState>,
) -> Result<Json<Vec<ConfigMeta>>, CommonError<String>> {
    let mut qb = QueryBuilder::new("SELECT * FROM config_meta WHERE 1=1");
    if let Some(code) = &filter.code {
        qb.push(" AND code=");
        qb.push_bind(code);
    }
    qb.push(" LIMIT ");
    qb.push_bind(pagination.limit);
    qb.push(" OFFSET ");
    qb.push_bind(pagination.offset);

    Ok(Json(
        qb.build_query_as::<ConfigMeta>()
            .fetch_all(&state.pool)
            .await?,
    ))
}

pub async fn get_raw(
    filter: &Filter,
    pagination: &Pagination,
    state: &AppState,
) -> Result<Vec<ConfigMeta>, sqlx::Error> {
    let mut qb = QueryBuilder::new("SELECT * FROM config_meta WHERE 1=1");
    if let Some(code) = &filter.code {
        qb.push(" AND code=");
        qb.push_bind(code);
    }
    qb.push(" LIMIT ");
    qb.push_bind(pagination.limit);
    qb.push(" OFFSET ");
    qb.push_bind(pagination.offset);

    qb.build_query_as::<ConfigMeta>()
        .fetch_all(&state.pool)
        .await
}

pub async fn get_one(
    Path(id): Path<i32>,
    Extension(state): Extension<AppState>,
) -> Result<Json<Option<ConfigMeta>>, CommonError<String>> {
    let mut qb = QueryBuilder::new("SELECT * FROM config_meta WHERE id=");
    qb.push_bind(id);

    Ok(Json(
        qb.build_query_as::<ConfigMeta>()
            .fetch_optional(&state.pool)
            .await?,
    ))
}

pub async fn post(
    Extension(state): Extension<AppState>,
    Json(input): Json<ConfigMeta>,
) -> Result<String, CommonError<String>> {
    let mut qb = QueryBuilder::new(
        "INSERT INTO config_meta(
        code, property, column_name, description, sort, 
        gmt_create, gmt_modified
        ) ",
    );

    qb.push_values([input], |mut s, i| {
        s.push_bind(i.code)
            .push_bind(i.property)
            .push_bind(i.column_name)
            .push_bind(i.description)
            .push_bind(i.sort)
            .push_bind(i.gmt_create)
            .push_bind(i.gmt_modified);
    });

    Ok(qb
        .build()
        .execute(&state.pool)
        .await?
        .last_insert_rowid()
        .to_string())
}

pub async fn put(
    Extension(state): Extension<AppState>,
    Path(id): Path<i32>,
    Json(input): Json<ConfigMetaUpdate>,
) -> Result<String, CommonError<String>> {
    let mut qb = QueryBuilder::new("UPDATE config_meta SET ");
    let mut has_value: bool = false;

    if let Some(v) = input.code {
        if has_value {
            qb.push(",");
        }
        qb.push(" code=");
        qb.push_bind(v);
        has_value = true;
    }
    if let Some(v) = input.property {
        if has_value {
            qb.push(",");
        }
        qb.push(" property=");
        qb.push_bind(v);
        has_value = true;
    }
    if let Some(v) = input.column_name {
        if has_value {
            qb.push(",");
        }
        qb.push(" column_name=");
        qb.push_bind(v);
        has_value = true;
    }
    if let Some(v) = input.description {
        if has_value {
            qb.push(",");
        }
        qb.push(" description=");
        qb.push_bind(v);
        has_value = true;
    }
    if let Some(v) = input.sort {
        if has_value {
            qb.push(",");
        }
        qb.push(" sort=");
        qb.push_bind(v);
        has_value = true;
    }

    if !has_value {
        return Err(CommonError {
            err: StatusCode::NOT_FOUND,
            data: Some(String::from("no record found")),
        });
    }

    qb.push(" WHERE id=");
    qb.push_bind(id);

    Ok(qb
        .build()
        .execute(&state.pool)
        .await?
        .rows_affected()
        .to_string())
}

pub async fn delete(
    Path(id): Path<i32>,
    Extension(state): Extension<AppState>,
) -> Result<String, CommonError<String>> {
    let mut qb = QueryBuilder::new("DELETE FROM config_meta WHERE id=");
    qb.push_bind(id);

    Ok(qb
        .build()
        .execute(&state.pool)
        .await?
        .rows_affected()
        .to_string())
}
