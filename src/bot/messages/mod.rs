mod any_messages;
mod commands;

use any_messages::messages_handler;
use commands::filter_commands;

use teloxide::{
    RequestError,
    dispatching::{UpdateFilterExt, UpdateHandler},
    dptree,
    types::{Message, Update},
};

fn main_filter(msg: Message) -> bool {
    msg.chat.is_private()
}

pub fn filter_message() -> UpdateHandler<RequestError> {
    Update::filter_message()
        .filter(main_filter)
        .branch(filter_commands())
        .branch(dptree::endpoint(messages_handler))
}
