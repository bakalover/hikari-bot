use teloxide::{prelude::*, utils::command::BotCommands};

use super::reply_text;

pub(super) async fn help<CmdType: BotCommands>(bot: Bot, msg: Message) -> ResponseResult<()> {
    reply_text(bot, msg, CmdType::descriptions().to_string()).await
}
