use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "author_is")]
pub enum AuthorIs {
    #[sea_orm(string_value = "User")]
    User,

    #[sea_orm(string_value = "Model")]
    Model,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "messages")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub message: i32,

    #[sea_orm(primary_key, auto_increment = false)]
    pub chat: i64,

    pub author: AuthorIs,
    pub text: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
