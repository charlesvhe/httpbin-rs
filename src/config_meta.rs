use axum::{
    extract::{Path, Query},
    Extension, Json, Router,
};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use serde::{Deserialize, Serialize};

use crate::{
    common::{AppState, CommonError, Pagination},
    entity::config_meta::{ActiveModel, Column, Entity, Model},
};

pub fn init_router() -> axum::Router {
    Router::new()
        .route("/ConfigMeta", axum::routing::get(get).post(post))
        .route(
            "/ConfigMeta/:id",
            axum::routing::get(get_one).put(put).delete(delete),
        )
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
) -> Result<Json<Vec<Model>>, CommonError<String>> {
    let mut select = Entity::find();

    if let Some(code) = &filter.code {
        select = select.filter(Column::Code.eq(code));
    }

    Ok(Json(
        select
            .offset(Some(pagination.offset as u64))
            .limit(Some(pagination.limit as u64))
            .all(&state.db)
            .await?,
    ))
}

pub async fn get_one(
    Path(id): Path<i32>,
    Extension(state): Extension<AppState>,
) -> Result<Json<Option<Model>>, CommonError<String>> {
    Ok(Json(Entity::find_by_id(id).one(&state.db).await?))
}

pub async fn post(
    Extension(state): Extension<AppState>,
    Json(input): Json<Model>,
) -> Result<Json<Model>, CommonError<String>> {
    Ok(Json(
        ActiveModel {
            id: ActiveValue::NotSet,
            code: ActiveValue::Set(input.code),
            property: ActiveValue::Set(input.property),
            column_name: ActiveValue::Set(input.column_name),
            description: ActiveValue::Set(input.description),
            sort: ActiveValue::Set(input.sort),
            gmt_create: ActiveValue::NotSet,
            gmt_modified: ActiveValue::NotSet,
        }
        .insert(&state.db)
        .await?,
    ))
}

pub async fn put(
    Extension(state): Extension<AppState>,
    Path(id): Path<i32>,
    Json(input): Json<ConfigMetaUpdate>,
) -> Result<Json<Model>, CommonError<String>> {
    Ok(Json(
        ActiveModel {
            id: ActiveValue::Set(id),
            code: input
                .code
                .map_or_else(|| ActiveValue::NotSet, |v| ActiveValue::Set(v)),
            property: input
                .property
                .map_or_else(|| ActiveValue::NotSet, |v| ActiveValue::Set(v)),
            column_name: input
                .column_name
                .map_or_else(|| ActiveValue::NotSet, |v| ActiveValue::Set(v)),
            description: input
                .description
                .map_or_else(|| ActiveValue::NotSet, |v| ActiveValue::Set(v)),
            sort: input
                .sort
                .map_or_else(|| ActiveValue::NotSet, |v| ActiveValue::Set(v)),
            gmt_create: ActiveValue::NotSet,
            gmt_modified: ActiveValue::NotSet,
        }
        .update(&state.db)
        .await?,
    ))
}

pub async fn delete(
    Path(id): Path<i32>,
    Extension(state): Extension<AppState>,
) -> Result<String, CommonError<String>> {
    Ok(Entity::delete_by_id(id)
        .exec(&state.db)
        .await?
        .rows_affected
        .to_string())
}
