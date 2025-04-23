use crate::chats;
use sea_orm::DbConn;
use teloxide::{
    Bot, RequestError,
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Message},
};

pub async fn start(bot: Bot, msg: Message, db: DbConn) -> Result<(), RequestError> {
    bot.send_message(msg.chat.id, "Выбери модель:")
        .reply_markup(InlineKeyboardMarkup::new(vec![vec![
            InlineKeyboardButton::callback("a".to_owned(), "b".to_owned()),
        ]]))
        .await?;

    Ok(())
}
