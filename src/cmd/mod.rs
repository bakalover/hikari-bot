use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub(crate) enum Command {
    #[command(description = "help <3")]
    Help,

    #[command(description = "search via")]
    Search(String),
}

pub(crate) async fn router(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .message_thread_id(msg.thread_id.unwrap())
                .await?
        }

        Command::Search(query) => {
            let reply = reqwest::get(format!(
                "https://jisho.org/api/v1/search/words?keyword={}",
                query
            ))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

            let truncated_message = if reply.len() > 4096 {
                &reply[..4096]
            } else {
                &reply
            };

            bot.send_message(msg.chat.id, truncated_message)
                .message_thread_id(msg.thread_id.unwrap())
                .await?
        }
    };

    Ok(())
}
