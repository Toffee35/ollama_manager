mod change;
mod clear;
mod generate;
mod help;
mod pull_req;
mod start;

use teloxide::utils::command::BotCommands;

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
