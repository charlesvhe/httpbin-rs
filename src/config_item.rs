use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};
use sqlx::QueryBuilder;

use crate::{
    common::{AppState, CommonError, Pagination},
    config_meta::{self, Filter},
};

pub fn init_router() -> axum::Router {
    Router::new()
        .route("/ConfigItem/:meta_code", axum::routing::get(get).post(post))
        .route(
            "/ConfigItem/:meta_code/:id",
            axum::routing::get(get_one).put(put).delete(delete),
        )
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct ConfigItem {
    pub id: i64,
    pub meta_code: String,
    pub parent_id: i64,
    pub sort: i32,
    pub varchar1: String,
    pub varchar2: String,
    pub varchar3: String,
    pub varchar4: String,
    pub varchar5: String,
    pub text1: String,
    pub text2: String,
    pub text3: String,
    pub decimal1: f64,
    pub decimal2: f64,
    pub decimal3: f64,
    pub datetime1: String,
    pub datetime2: String,
    pub datetime3: String,
    pub gmt_create: String,
    pub gmt_modified: String,
}

pub async fn get(
    Path(meta_code): Path<String>,
    pagination: Query<Pagination>,
    Extension(state): Extension<AppState>,
) -> Result<Json<Vec<Map<String, Value>>>, CommonError<String>> {
    let column_property_map = get_column_property_map(meta_code.clone(), &state).await;

    let mut qb = QueryBuilder::new("SELECT * FROM config_item WHERE meta_code=");
    qb.push_bind(meta_code);

    qb.push(" LIMIT ");
    qb.push_bind(pagination.limit);

    qb.push(" OFFSET ");
    qb.push_bind(pagination.offset);

    let res = qb
        .build_query_as::<ConfigItem>()
        .fetch_all(&state.pool)
        .await?;

    Ok(Json(
        res.into_iter()
            .map(|ci| convert(&column_property_map, ci))
            .collect::<Vec<Map<String, Value>>>(),
    ))
}

pub async fn get_one(
    Path((meta_code, id)): Path<(String, i32)>,
    Extension(state): Extension<AppState>,
) -> Result<Json<Map<String, Value>>, CommonError<String>> {
    let column_property_map = get_column_property_map(meta_code.clone(), &state).await;
    
    let mut qb = QueryBuilder::new("SELECT * FROM config_item WHERE meta_code=");
    qb.push_bind(meta_code);

    qb.push(" AND id=");
    qb.push_bind(id);

    if let Some(ci) = qb
        .build_query_as::<ConfigItem>()
        .fetch_optional(&state.pool)
        .await?
    {
        return Ok(Json(convert(&column_property_map, ci)));
    } else {
        return Err(CommonError {
            err: StatusCode::NOT_FOUND,
            data: Some(String::from("no record found")),
        });
    }
}

async fn get_column_property_map(meta_code: String, state: &AppState) -> HashMap<String, String> {
    let vec_config_meta = config_meta::get(
        Query(Pagination {
            offset: 0,
            limit: 100,
        }),
        Query(Filter {
            code: Some(meta_code),
        }),
        Extension(state.clone()),
    )
    .await;

    let mut column_property_map = HashMap::<String, String>::new();

    if let Ok(items) = vec_config_meta {
        items.0.into_iter().for_each(|cm| {
            column_property_map.insert(cm.column_name, cm.property);
        });
    }

    return column_property_map;
}

fn convert(column_property_map: &HashMap<String, String>, ci: ConfigItem) -> Map<String, Value> {
    let mut res = Map::new();

    [
        ("varchar1", Value::String(ci.varchar1)),
        ("varchar2", Value::String(ci.varchar2)),
        ("varchar3", Value::String(ci.varchar3)),
        ("varchar4", Value::String(ci.varchar4)),
        ("varchar5", Value::String(ci.varchar5)),
        ("text1", Value::String(ci.text1)),
        ("text2", Value::String(ci.text2)),
        ("text3", Value::String(ci.text3)),
        (
            "decimal1",
            match Number::from_f64(ci.decimal1) {
                Some(v) => Value::Number(v),
                None => Value::Null,
            },
        ),
        (
            "decimal2",
            match Number::from_f64(ci.decimal2) {
                Some(v) => Value::Number(v),
                None => Value::Null,
            },
        ),
        (
            "decimal3",
            match Number::from_f64(ci.decimal3) {
                Some(v) => Value::Number(v),
                None => Value::Null,
            },
        ),
        ("datetime1", Value::String(ci.datetime1)),
        ("datetime2", Value::String(ci.datetime2)),
        ("datetime3", Value::String(ci.datetime3)),
    ]
    .into_iter()
    .for_each(|(column, value)| {
        if let Some(property) = column_property_map.get(column) {
            res.insert(property.to_string(), value);
        }
    });

    res
}

pub async fn post(
    Extension(state): Extension<AppState>,
    Path(meta_code): Path<String>,
    Json(mut input): Json<ConfigItem>,
) -> Result<String, CommonError<String>> {
    input.meta_code = meta_code;

    let mut qb = QueryBuilder::new(
        "INSERT INTO config_item(
        meta_code, parent_id, sort, 
        varchar1, varchar2, varchar3, varchar4, varchar5,
        text1, text2, text3,
        decimal1, decimal2, decimal3,
        datetime1, datetime2, datetime3,
        gmt_create, gmt_modified
        ) ",
    );

    qb.push_values([input], |mut s, i| {
        s.push_bind(i.meta_code)
            .push_bind(i.parent_id)
            .push_bind(i.sort)
            .push_bind(i.varchar1)
            .push_bind(i.varchar2)
            .push_bind(i.varchar3)
            .push_bind(i.varchar4)
            .push_bind(i.varchar5)
            .push_bind(i.text1)
            .push_bind(i.text2)
            .push_bind(i.text3)
            .push_bind(i.decimal1)
            .push_bind(i.decimal2)
            .push_bind(i.decimal3)
            .push_bind(i.datetime1)
            .push_bind(i.datetime2)
            .push_bind(i.datetime3)
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

pub async fn put() -> impl IntoResponse {
    Json("")
}

pub async fn delete(
    Path((meta_code, id)): Path<(String, i32)>,
    Extension(state): Extension<AppState>,
) -> Result<String, CommonError<String>> {
    let mut qb = QueryBuilder::new("DELETE FROM config_item WHERE meta_code=");
    qb.push_bind(meta_code);

    qb.push(" AND id=");
    qb.push_bind(id);

    Ok(qb
        .build()
        .execute(&state.pool)
        .await?
        .rows_affected()
        .to_string())
}
