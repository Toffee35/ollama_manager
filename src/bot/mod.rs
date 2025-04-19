mod messages;

pub use messages::filter_message;

use std::env;
use teloxide::{Bot, RequestError, dispatching::UpdateHandler, dptree};

pub fn make_bot() -> Bot {
    let token: String = env::var("BOT_TOKEN").unwrap();

    Bot::new(token)
}

pub fn make_handler() -> UpdateHandler<RequestError> {
    dptree::entry().branch(filter_message())
}
