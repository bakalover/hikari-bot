mod check;
mod db;
mod info;
mod kana;

use std::sync::Arc;

use crate::HandlerResult;
use teloxide::{
    requests::Requester,
    types::{ChatId, Message},
    Bot,
};
use tokio_postgres::{Client, NoTls};

enum State {
    Continue,
    Stop,
}

type RoundResult = Result<State, Box<dyn std::error::Error + Send + Sync>>;

async fn firs_bot_round(bot: &Bot, chat_id: ChatId, client: &Client) -> HandlerResult {
    let word = "雲";
    db::add_word(word, client).await;
    bot.send_message(chat_id, format!("Стартовое слово: {word} \n"))
        .await?;
    info::send_next_word_info(&bot, chat_id, word).await?;
    Ok(())
}

pub(crate) async fn game(bot: Bot, start_message: Message) -> HandlerResult {
    let (client, _) = tokio_postgres::connect("host=localhost user=bakalover", NoTls)
        .await
        .unwrap();

    let chat_id = start_message.chat.id;
    info::say_hi(&bot, chat_id).await?;
    firs_bot_round(&bot, chat_id, &client).await?;

    loop {
        match play_round(&bot, chat_id, &client).await? {
            State::Continue => (),
            State::Stop => {
                info::send_statistics(&bot, chat_id).await?;
                break;
            }
        };
    }

    Ok(())
}

async fn play_round(bot: &Bot, chat_id: ChatId, client: &Client) -> RoundResult {
    let word = db::get_last(client).await; // Obtain last word

    if check::check_for_stop(word.as_str()) {
        return Ok(State::Stop);
    }

    if check::check_n_ending(word.as_str()) {
        info::say_about_n_ending(&bot, chat_id).await?;
        return Ok(State::Stop);
    }

    if check::check_for_twice(word.as_str()) {
        info::say_about_twice(&bot, chat_id).await?;
        return Ok(State::Continue);
    }

    // bot.send_message(chat_id, word).await?;
    // info::send_next_word_info(&bot, chat_id, word).await?;
    Ok(State::Continue)
}
