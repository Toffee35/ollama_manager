mod delete;
mod make;
mod pull;

use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Admin {
    Delete,
    Make,
    Pull,
}
