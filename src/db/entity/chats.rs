use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "chat_state")]
pub enum State {
    #[sea_orm(string_value = "Selecting")]
    Selecting,

    #[sea_orm(string_value = "Waiting")]
    Waiting,

    #[sea_orm(string_value = "Chating")]
    Chating,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "chats")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub chat: i64,

    pub state: State,

    pub model: String,
    pub variant: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
