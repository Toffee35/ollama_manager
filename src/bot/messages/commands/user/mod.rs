mod change;
mod clear;
mod generate;
mod help;
mod pull_req;
mod start;

use sea_orm::DbConn;
use teloxide::{
    Bot, RequestError, prelude::Requester, types::Message, utils::command::BotCommands,
};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum User {
    Change,
    Clear,
    Gen,
    Help,
    PullReq,
    Start,
}

pub async fn user_handler(
    bot: Bot,
    cmd: User,
    msg: Message,
    db: DbConn,
) -> Result<(), RequestError> {
    match cmd {
        User::Start => start::start(bot, msg, db).await?,
        _ => {
            bot.send_message(msg.chat.id, "User - Some").await?;
        }
    }

    Ok(())
}
