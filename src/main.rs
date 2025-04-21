mod bot;
mod db;
mod models;

use bot::{make_bot, make_handler};
use db::{chats, make_db, messages, users};
use models::{Models, make_models};

use sea_orm::DbConn;
use teloxide::{
    Bot, RequestError,
    dispatching::{Dispatcher, UpdateHandler},
    dptree,
};

#[tokio::main]
async fn main() {
    let db: DbConn = make_db().await;

    let models: Models = make_models().await;

    let bot: Bot = make_bot();

    let handler: UpdateHandler<RequestError> = make_handler();

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![db, models])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
