pub mod commands;
pub mod jisho;

use commands::Command;
use jisho::search_word;
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer_handler).await;
}

async fn answer_handler(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Search(request) => {
            bot.send_message(msg.chat.id, search_word(request).await.unwrap())
                .await?
        }
    };
    Ok(())
}
