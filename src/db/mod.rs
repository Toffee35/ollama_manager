mod entity;

use entity::create_tables;

pub use entity::{chats, messages};

use sea_orm::{ConnectOptions, Database, DbConn};
use std::env;

pub async fn make_db() -> DbConn {
    let url: String = env::var("DATABASE_URL").unwrap();

    let opt: ConnectOptions = ConnectOptions::new(url);

    let db: DbConn = Database::connect(opt).await.unwrap();

    create_tables(&db).await;

    db
}
