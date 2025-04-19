pub mod chats;
pub mod messages;

use sea_orm::{
    ConnectionTrait, DbBackend, DbConn, DbErr, RuntimeErr, Schema, SqlxError,
    sea_query::{TableCreateStatement, extension::postgres::TypeCreateStatement},
};

async fn create_enums(db: &DbConn) -> Result<(), DbErr> {
    let backend: DbBackend = db.get_database_backend();

    let schema = Schema::new(backend);

    let chat_state_enums: &Vec<TypeCreateStatement> =
        &schema.create_enum_from_entity(chats::Entity);

    for chat_state_enum in chat_state_enums {
        let result = db.execute(backend.build(chat_state_enum)).await;
        match result {
            Ok(_) => (),
            Err(err) => match &err {
                DbErr::Exec(RuntimeErr::SqlxError(SqlxError::Database(db_err))) => {
                    if db_err.code().as_deref() == Some("42710") {
                        continue;
                    } else {
                        return Err(err);
                    }
                }
                _ => return Err(err),
            },
        }
    }

    let message_enums: &Vec<TypeCreateStatement> =
        &schema.create_enum_from_entity(messages::Entity);

    for message_enum in message_enums {
        let result = db.execute(backend.build(message_enum)).await;
        match result {
            Ok(_) => (),
            Err(err) => match &err {
                DbErr::Exec(RuntimeErr::SqlxError(SqlxError::Database(db_err))) => {
                    if db_err.code().as_deref() == Some("42710") {
                        continue;
                    } else {
                        return Err(err);
                    }
                }
                _ => return Err(err),
            },
        }
    }

    Ok(())
}

pub async fn create_tables(db: &DbConn) {
    let backend: DbBackend = db.get_database_backend();

    let schema = Schema::new(backend);

    create_enums(&db).await.unwrap();

    let chat_state_statement: &TableCreateStatement = &schema
        .create_table_from_entity(chats::Entity)
        .if_not_exists()
        .to_owned();

    db.execute(backend.build(chat_state_statement))
        .await
        .unwrap();

    let message_statement: &TableCreateStatement = &schema
        .create_table_from_entity(messages::Entity)
        .if_not_exists()
        .to_owned();

    db.execute(backend.build(message_statement)).await.unwrap();
}
