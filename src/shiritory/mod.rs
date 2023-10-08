use teloxide::{
    requests::{Request, Requester},
    types::{ChatId, Message},
    Bot,
};

use crate::HandlerResult;

async fn say_hi(bot: &Bot, chat_id: ChatId) -> HandlerResult {
    let message = "Да начнётся игра в ширитори! \n Для остановки напишите \"shiritory stop\" в чат";
    bot.send_message(chat_id, message).send().await?;
    Ok(())
}

fn is_need_to_stop(msg: &str) -> bool {
    return msg == "shiritory stop";
}

pub(crate) async fn start(bot: Bot, msg: Message) -> HandlerResult {
    say_hi(&bot, msg.chat.id).await?;
    //Вставляем первое слово в бд
    
    loop {
        play_round(&bot, &msg);
    }

    Ok(())
}

fn play_round(bot: &Bot, msg: &Message) {
    // Выбираем последнее слово
    // Пишем, чтобы вводили слово начинающееся с концовки последнего из бд
    // В хэндлере проверяем сигнатуру игрока и при новом ответе говорим ему приветственное
    // teloxide::repl(bot, handler);
}
