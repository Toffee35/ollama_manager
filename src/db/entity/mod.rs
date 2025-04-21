pub mod chats;
pub mod messages;
pub mod users;

use sea_orm::{
    ConnectionTrait, DbBackend, DbConn, DbErr, EntityTrait, RuntimeErr, Schema, SqlxError,
    sea_query::{TableCreateStatement, extension::postgres::TypeCreateStatement},
};

pub async fn create_enums(db: &DbConn, entry: impl EntityTrait) -> Result<(), DbErr> {
    let backend: DbBackend = db.get_database_backend();

    let schema = Schema::new(backend);

    let enums: &Vec<TypeCreateStatement> = &schema.create_enum_from_entity(entry);

    for statement in enums {
        let result = db.execute(backend.build(statement)).await;
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

pub async fn create_tables(db: &DbConn, entry: impl EntityTrait) {
    let backend: DbBackend = db.get_database_backend();

    let schema = Schema::new(backend);

    let statement: &TableCreateStatement = &schema
        .create_table_from_entity(entry)
        .if_not_exists()
        .to_owned();

    db.execute(backend.build(statement)).await.unwrap();
}
