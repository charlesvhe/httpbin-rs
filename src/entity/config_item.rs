//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, DeriveEntityModel)]
#[serde(default)]
#[sea_orm(table_name = "config_item")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub meta_code: String,
    pub parent_id: i32,
    pub sort: i32,
    pub varchar1: String,
    pub varchar2: String,
    pub varchar3: String,
    pub varchar4: String,
    pub varchar5: String,
    pub text1: Option<String>,
    pub text2: Option<String>,
    pub text3: Option<String>,
    #[sea_orm(column_type = "Double")]
    pub decimal1: f64,
    #[sea_orm(column_type = "Double")]
    pub decimal2: f64,
    #[sea_orm(column_type = "Double")]
    pub decimal3: f64,
    pub datetime1: DateTime,
    pub datetime2: DateTime,
    pub datetime3: DateTime,
    pub gmt_create: DateTime,
    pub gmt_modified: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
