mod entity;

use entity::{create_enums, create_tables};

pub use entity::{chats, messages, users};

use sea_orm::{ConnectOptions, Database, DbConn};
use std::env;

pub async fn make_db() -> DbConn {
    let url: String = env::var("DATABASE_URL").unwrap();

    let opt: ConnectOptions = ConnectOptions::new(url);

    let db: DbConn = Database::connect(opt).await.unwrap();

    create_enums(&db, chats::Entity).await.unwrap();
    create_tables(&db, chats::Entity).await;

    create_enums(&db, messages::Entity).await.unwrap();
    create_tables(&db, messages::Entity).await;

    create_enums(&db, users::Entity).await.unwrap();
    create_tables(&db, users::Entity).await;

    db
}
