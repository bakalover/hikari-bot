mod cmd;

use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let bot = Bot::from_env();

    cmd::Command::repl(bot, cmd::router).await;
}
