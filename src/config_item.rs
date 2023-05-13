use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Extension, Json, Router,
};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use serde_json::{Map, Value};

use crate::{
    common::{AppState, CommonError, Pagination},
    entity::{
        self,
        config_item::{ActiveModel, Column, Entity, Model},
    },
};

pub fn init_router() -> axum::Router {
    Router::new()
        .route("/ConfigItem/:meta_code", axum::routing::get(get).post(post))
        .route(
            "/ConfigItem/:meta_code/:id",
            axum::routing::get(get_one).put(put).delete(delete),
        )
}

pub async fn get(
    Path(meta_code): Path<String>,
    pagination: Query<Pagination>,
    Extension(state): Extension<AppState>,
) -> Result<Json<Vec<Map<String, Value>>>, CommonError<String>> {
    let column_property_map = get_column_property_map(meta_code.clone(), &state).await;

    let res = Entity::find()
        .filter(Column::MetaCode.eq(meta_code))
        .offset(Some(pagination.offset as u64))
        .limit(Some(pagination.limit as u64))
        .all(&state.db)
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

    if let Some(ci) = Entity::find()
        .filter(Column::Id.eq(id))
        .filter(Column::MetaCode.eq(meta_code))
        .one(&state.db)
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
    let mut column_property_map = HashMap::<String, String>::new();

    column_property_map.insert("id".to_string(), "id".to_string());
    column_property_map.insert("parent_id".to_string(), "parent_id".to_string());
    column_property_map.insert("sort".to_string(), "sort".to_string());
    column_property_map.insert("gmt_create".to_string(), "gmt_create".to_string());
    column_property_map.insert("gmt_modified".to_string(), "gmt_modified".to_string());

    let vec_config_meta = entity::config_meta::Entity::find()
        .filter(entity::config_meta::Column::Code.eq(meta_code))
        .all(&state.db)
        .await;

    if let Ok(items) = vec_config_meta {
        for item in items {
            column_property_map.insert(item.column_name, item.property);
        }
    }

    return column_property_map;
}

fn convert(column_property_map: &HashMap<String, String>, model: Model) -> Map<String, Value> {
    let mut res = Map::new();
    [
        ("id", Value::from(model.id)),
        ("parent_id", Value::from(model.parent_id)),
        ("sort", Value::from(model.sort)),
        ("gmt_create", Value::from(model.gmt_create.to_string())),
        ("gmt_modified", Value::from(model.gmt_modified.to_string())),

        ("varchar1", Value::from(model.varchar1)),
        ("varchar2", Value::from(model.varchar2)),
        ("varchar3", Value::from(model.varchar3)),
        ("varchar4", Value::from(model.varchar4)),
        ("varchar5", Value::from(model.varchar5)),
        ("text1", model.text1.map_or_else(|| Value::Null, |v| Value::from(v))),
        ("text2", model.text2.map_or_else(|| Value::Null, |v| Value::from(v))),
        ("text3", model.text3.map_or_else(|| Value::Null, |v| Value::from(v))),
        ("decimal1", Value::from(model.decimal1)),
        ("decimal1", Value::from(model.decimal2)),
        ("decimal1", Value::from(model.decimal3)),
        ("datetime1", Value::from(model.datetime1.to_string())),
        ("datetime1", Value::from(model.datetime2.to_string())),
        ("datetime1", Value::from(model.datetime3.to_string())),
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
    body: String,
) -> Result<Json<Map<String, Value>>, CommonError<String>> {
    if let Ok(Value::Object(property_value_map)) = serde_json::from_str::<Value>(&body) {
        let mut column_value_map = serde_json::Map::new();

        let column_property_map = get_column_property_map(meta_code.clone(), &state).await;
        for (column, property) in column_property_map.iter() {
            if let Some(value) = property_value_map.get(property) {
                column_value_map.insert(column.clone(), value.clone());
            }
        }

        if let Ok(mut model) = ActiveModel::from_json(Value::Object(column_value_map)) {
            model.meta_code = ActiveValue::Set(meta_code);
            return Ok(Json(convert(
                &column_property_map,
                model.insert(&state.db).await?,
            )));
        }
    }

    Err(CommonError {
        err: StatusCode::BAD_REQUEST,
        data: Some(String::from("bad json")),
    })
}

pub async fn put(
    Extension(state): Extension<AppState>,
    Path((meta_code, id)): Path<(String, i32)>,
    body: String,
) -> Result<Json<Map<String, Value>>, CommonError<String>> {
    if let Ok(Value::Object(property_value_map)) = serde_json::from_str::<Value>(&body) {
        let mut column_value_map = serde_json::Map::new();

        let column_property_map = get_column_property_map(meta_code.clone(), &state).await;
        for (column, property) in column_property_map.iter() {
            if let Some(value) = property_value_map.get(property) {
                column_value_map.insert(column.clone(), value.clone());
            }
        }

        if let Ok(mut model) = ActiveModel::from_json(Value::Object(column_value_map)) {
            model.id = ActiveValue::Set(id);
            model.meta_code = ActiveValue::Set(meta_code);
            return Ok(Json(convert(
                &column_property_map,
                model.update(&state.db).await?,
            )));
        }
    }

    Err(CommonError {
        err: StatusCode::BAD_REQUEST,
        data: Some(String::from("bad json")),
    })
}

pub async fn delete(
    Path((meta_code, id)): Path<(String, i32)>,
    Extension(state): Extension<AppState>,
) -> Result<String, CommonError<String>> {
    Ok(Entity::delete_by_id(id)
        .filter(Column::MetaCode.eq(meta_code))
        .exec(&state.db)
        .await?
        .rows_affected
        .to_string())
}
