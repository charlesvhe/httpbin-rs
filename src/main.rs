use axum::{Extension, Router};
use sea_orm::{Database, ConnectionTrait, Statement, DatabaseBackend};
use std::{fs, net::SocketAddr};

use crate::common::AppState;

pub mod entity;
pub mod common;
pub mod config_meta;
pub mod config_item;

#[tokio::main]
async fn main() {
    let db = Database::connect("sqlite::memory:").await.unwrap();
    let app_state = AppState {
        db: db,
    };

    app_state.db.execute(Statement::from_string(
        DatabaseBackend::Sqlite,
        String::from_utf8(fs::read("db/migration/V1_0__schema.sql").unwrap()).ok().unwrap()
    )).await.unwrap();

    // build our application with a route
    let router = Router::new()
        .merge(config_meta::init_router())
        .merge(config_item::init_router())
        .layer(Extension(app_state));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
