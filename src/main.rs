pub mod commands;
pub mod jisho;
pub mod shiritory;

use commands::Command;
use teloxide::{
    dispatching::{DispatcherBuilder, UpdateHandler},
    prelude::*,
    utils::command::BotCommands,
};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env();
    // Dispatcher::builder(bot)
}

// fn answer_handler() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
//     use dptree::case;

//     let command_handler = teloxide::filter_command::<Command, _>()
//         .branch(case![Command::Help].endpoint(help))
//         .branch(case![Command::Search(req)].endpoint(jisho::search_word))
//         .branch(case![Command::Shiritory].endpoint(shiritory::start));

//     // teloxide::dispatching::dialogue::enter()
//     //     .branch(command_handler)
// }

async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}
