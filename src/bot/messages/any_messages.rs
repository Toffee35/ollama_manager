use sea_orm::DbConn;
use teloxide::{RequestError, types::Message};

pub async fn messages_handler(msg: Message, db: DbConn) -> Result<(), RequestError> {
    Ok(())
}
