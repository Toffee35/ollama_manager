mod admin;
mod user;

use admin::Admin;
use user::User;

use sea_orm::DbConn;
use teloxide::{
    RequestError,
    dispatching::{HandlerExt, UpdateHandler},
    dptree,
    types::Message,
};

async fn user_handler(msg: Message, _db: DbConn) -> Result<(), RequestError> {
    if let Some(text) = msg.text() {
        println!("User command: {}", text);
    }

    Ok(())
}

async fn admin_handler(msg: Message, _db: DbConn) -> Result<(), RequestError> {
    if let Some(text) = msg.text() {
        println!("Admin command: {}", text);
    }

    Ok(())
}

pub fn filter_commands() -> UpdateHandler<RequestError> {
    dptree::entry()
        .branch(
            dptree::entry()
                .filter_command::<User>()
                .endpoint(user_handler),
        )
        .branch(
            dptree::entry()
                .filter_command::<Admin>()
                .endpoint(admin_handler),
        )
}
