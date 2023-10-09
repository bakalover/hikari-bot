pub mod commands;
pub mod jisho;
pub mod shiritory;

use commands::Command;
use teloxide::{dispatching::UpdateHandler, prelude::*, utils::command::BotCommands};
use tokio_postgres::{Client, NoTls};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");
    let (client, _) = tokio_postgres::connect("host=localhost user=bakalover", NoTls)
        .await
        .unwrap();

    let bot = Bot::from_env();
    Dispatcher::builder(bot, command_handler(client))
        .dependencies(dptree::deps![client])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn command_handler(
    client: Client,
) -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    teloxide::filter_command::<Command, _>()
        .branch(case![Command::Help].endpoint(help))
        .branch(case![Command::Search(req)].endpoint(jisho::search_word))
        .branch(case![Command::Shiritory].endpoint(|bot, msg| shiritory::game(bot.clone(), msg, client)))
}

async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}
