use axum::{Extension, Router};
use sqlx::SqlitePool;
use std::{fs, net::SocketAddr};

use crate::common::AppState;

pub mod common;
pub mod config_meta;
pub mod config_item;

#[tokio::main]
async fn main() {
    let app_state = AppState {
        pool: SqlitePool::connect("sqlite::memory:").await.unwrap(),
    };

    // init database
    sqlx::query(&String::from_utf8(fs::read("db/migration/V1_0__schema.sql").unwrap()).unwrap())
        .execute(&app_state.pool)
        .await
        .unwrap();

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
