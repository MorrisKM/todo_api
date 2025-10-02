use sea_orm::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Default, Serialize)]
#[sea_orm(table_name = "todos")]

pub struct Model {
  #[sea_orm(primary_key)]
  id: i32,
  content: String,
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}


// run postgres:
// 1. sudo su postgres
// 2. psql
// 3. \c  to select table