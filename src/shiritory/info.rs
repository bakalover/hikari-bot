use teloxide::{
    requests::{Request, Requester},
    types::ChatId,
    Bot,
};

use crate::HandlerResult;

use super::kana;

pub(super) async fn say_hi(bot: &Bot, chat_id: ChatId) -> HandlerResult {
    let message = "Да начнётся игра в ширитори! \n Для остановки напишите \"shiritory stop\" в чат";
    bot.send_message(chat_id, message).send().await?;
    Ok(())
}

pub(super) async fn say_about_n_ending(bot: &Bot, chat_id: ChatId) -> HandlerResult {
    bot.send_message(chat_id, format!("Слово заканчивающееся на ん！\n"))
        .await?;
    Ok(())
}

pub(super) async fn say_about_twice(bot: &Bot, chat_id: ChatId) -> HandlerResult {
    bot.send_message(chat_id, format!("Повторяющееся слово!\n"))
        .await?;
    Ok(())
}

pub(super) async fn send_next_word_info(
    bot: &Bot,
    chat_id: ChatId,
    previous_word: &str, // Already validated
) -> HandlerResult {
    let start_kana = kana::get_start_kana(previous_word);
    let next = format!("Следующее слово начинается с {start_kana} \n");
    bot.send_message(chat_id, next).await?;
    Ok(())
}

pub(super) async fn send_statistics(bot: &Bot, chat_id: ChatId) -> HandlerResult {
    todo!()
}
