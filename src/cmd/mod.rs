use help::help;
use search::search;
use teloxide::{prelude::*, utils::command::BotCommands};

use crate::search::{jisho::Jisho, jmdict::JMDict};

mod help;
mod search;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Команды:")]
pub(crate) enum MainCmd {
    #[command(description = "справка по командам")]
    Help,

    #[command(description = "справка по командам ширитори")]
    Shiritory_Help,

    #[command(description = "справка по командам админов")]
    Admin_Help,

    #[command(description = "\"/jisho <слово/кандзи>\" [поиск Jisho | EN]")]
    Jisho(String),

    #[command(description = "\"/jmdict <слово/кандзи>\" [поиск JMDict | RU]")]
    JMDict(String),

    #[command(description = "подтвердить участие в ближайшей встрече")]
    Apply_Meeting,

    #[command(description = "отказаться от участия в ближайшей встрече")]
    Discard_Meeting,

    #[command(description = "слово дня")]
    Daily_Word,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub(crate) enum AdminCmd {
    #[command(description = "опубликовать пост")]
    Post,

    #[command(description = "настройки постов")]
    Configure_Post,
}

#[derive(BotCommands, Clone)] // Bugged unsafe inside macros
#[command(rename_rule = "lowercase")]
pub(crate) enum ShiritoryCmd {
    #[command(description = "начать игру в ширитори")]
    Shiritory(String), // thread-pinned FSM + In-memory state
}

async fn reply_text<T: Into<String>>(bot: Bot, msg: Message, text: T) -> ResponseResult<()> {
    let send = bot.send_message(msg.chat.id, text);
    if msg.thread_id.is_some() {
        // subchat
        send.message_thread_id(msg.thread_id.unwrap()).await?;
    } else {
        // single thread group
        send.await?;
    }
    Ok(())
}

pub(crate) async fn router(bot: Bot, msg: Message, cmd: MainCmd) -> ResponseResult<()> {
    match cmd {
        MainCmd::Help => help::<MainCmd>(bot, msg).await,
        MainCmd::Shiritory_Help => help::<ShiritoryCmd>(bot, msg).await,
        MainCmd::Admin_Help => help::<AdminCmd>(bot, msg).await,

        MainCmd::Jisho(query) => search::<Jisho>(bot, msg, query.as_str()).await,
        MainCmd::JMDict(query) => search::<JMDict>(bot, msg, query.as_str()).await,

        _ => todo!(),
    }
}
