mod cmd;
mod search;

use teloxide::prelude::*;

#[tokio::main(flavor = "multi_thread", worker_threads = 3)]
async fn main() {
    pretty_env_logger::init();
    let bot = Bot::from_env();

    cmd::MainCmd::repl(bot, cmd::router).await;
}
