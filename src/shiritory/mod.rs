mod check;
mod info;
mod kana;

use crate::HandlerResult;
use teloxide::{
    requests::Requester,
    types::{ChatId, Message},
    Bot,
};
use tokio_postgres::Client;

enum State {
    Continue,
    Stop,
}

type RoundResult = Result<State, Box<dyn std::error::Error + Send + Sync>>;

async fn firs_bot_round(bot: &Bot, chat_id: ChatId) -> HandlerResult {
    // Get random word and insert into into db
    let word = "雲"; // Special display option with meaning and hiragana reading
    let first_word = format!("Стартовое слово: {word} \n");
    bot.send_message(chat_id, first_word).await?;
    info::send_next_word_info(&bot, chat_id, word).await?;
    Ok(())
}

pub(crate) async fn game(bot: Bot, start_message: Message, client: Client) -> HandlerResult {
    let chat_id = start_message.chat.id;
    info::say_hi(&bot, chat_id).await?;
    firs_bot_round(&bot, chat_id).await?;

    loop {
        match play_round(&bot, chat_id).await? {
            State::Continue => (),
            State::Stop => {
                info::send_statistics(&bot, chat_id).await?;
                break;
            }
        };
    }

    Ok(())
}

async fn play_round(bot: &Bot, chat_id: ChatId) -> RoundResult {
    let word = todo!(); // Obtain last word

    if check::check_for_stop(word) {
        return Ok(State::Stop);
    }

    if check::check_n_ending(word) {
        info::say_about_n_ending(&bot, chat_id).await?;
        return Ok(State::Stop);
    }

    if check::check_for_twice(word) {
        info::say_about_twice(&bot, chat_id).await?;
        return Ok(State::Continue);
    }

    // bot.send_message(chat_id, word).await?;
    // info::send_next_word_info(&bot, chat_id, word).await?;
    Ok(State::Continue)
}
