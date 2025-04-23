use crate::chats;
use sea_orm::DbConn;
use teloxide::{
    Bot, RequestError,
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{KeyboardButton, KeyboardMarkup, Message},
};

pub async fn start(bot: Bot, msg: Message, db: DbConn) -> Result<(), RequestError> {
    bot.send_message(msg.chat.id, "Выбери модель:")
        .reply_markup(
            KeyboardMarkup::new(vec![
                vec![
                    KeyboardButton::new("1a"),
                    KeyboardButton::new("2a"),
                    KeyboardButton::new("3a"),
                ],
                vec![
                    KeyboardButton::new("1b"),
                    KeyboardButton::new("2b"),
                    KeyboardButton::new("3b"),
                ],
                vec![
                    KeyboardButton::new("1c"),
                    KeyboardButton::new("2c"),
                    KeyboardButton::new("3c"),
                ],
            ])
            .resize_keyboard()
            .one_time_keyboard(),
        )
        .await?;

    Ok(())
}
