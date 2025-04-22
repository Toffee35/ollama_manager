mod delete;
mod make;
mod pull;

use sea_orm::DbConn;
use teloxide::{
    Bot, RequestError, prelude::Requester, types::Message, utils::command::BotCommands,
};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Admin {
    Delete,
    Make,
    Pull,
}

pub async fn admin_handler(
    bot: Bot,
    cmd: Admin,
    msg: Message,
    _db: DbConn,
) -> Result<(), RequestError> {
    bot.send_message(
        msg.chat.id,
        format!(
            "Admin - {}",
            match cmd {
                Admin::Delete => "Delete",
                Admin::Make => "Make",
                Admin::Pull => "Pull",
            }
        ),
    )
    .await?;

    Ok(())
}
