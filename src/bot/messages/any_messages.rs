use sea_orm::DbConn;
use teloxide::{RequestError, types::Message};

pub async fn messages_handler(msg: Message, _db: DbConn) -> Result<(), RequestError> {
    if let Some(text) = msg.text() {
        println!("Just message: {}", text);
    }

    Ok(())
}
