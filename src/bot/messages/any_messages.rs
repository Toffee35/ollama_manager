use sea_orm::DbConn;
use teloxide::{Bot, RequestError, prelude::Requester, types::Message};

pub async fn messages_handler(bot: Bot, msg: Message, _db: DbConn) -> Result<(), RequestError> {
    bot.send_message(msg.chat.id, "Message").await?;

    Ok(())
}
