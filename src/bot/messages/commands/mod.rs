mod admin;
mod user;

use admin::{Admin, admin_handler};
use user::{User, user_handler};

use teloxide::{
    RequestError,
    dispatching::{HandlerExt, UpdateHandler},
    dptree,
};

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
