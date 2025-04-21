pub mod chats;
pub mod messages;
pub mod users;

use sea_orm::{
    ConnectionTrait, DbBackend, DbConn, DbErr, RuntimeErr, Schema, SqlxError,
    sea_query::{TableCreateStatement, extension::postgres::TypeCreateStatement},
};

async fn create_enums(db: &DbConn) -> Result<(), DbErr> {
    let backend: DbBackend = db.get_database_backend();

    let schema = Schema::new(backend);

    let chats_enums: &Vec<TypeCreateStatement> = &schema.create_enum_from_entity(chats::Entity);

    for chats_enum in chats_enums {
        let result = db.execute(backend.build(chats_enum)).await;
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

    let messages_enums: &Vec<TypeCreateStatement> =
        &schema.create_enum_from_entity(messages::Entity);

    for messages_enum in messages_enums {
        let result = db.execute(backend.build(messages_enum)).await;
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

    let users_enums: &Vec<TypeCreateStatement> = &schema.create_enum_from_entity(users::Entity);

    for users_enum in users_enums {
        let result = db.execute(backend.build(users_enum)).await;
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

    let chats_statement: &TableCreateStatement = &schema
        .create_table_from_entity(chats::Entity)
        .if_not_exists()
        .to_owned();

    db.execute(backend.build(chats_statement)).await.unwrap();

    let messages_statement: &TableCreateStatement = &schema
        .create_table_from_entity(messages::Entity)
        .if_not_exists()
        .to_owned();

    db.execute(backend.build(messages_statement)).await.unwrap();

    let users_statement: &TableCreateStatement = &schema
        .create_table_from_entity(users::Entity)
        .if_not_exists()
        .to_owned();

    db.execute(backend.build(users_statement)).await.unwrap();
}
