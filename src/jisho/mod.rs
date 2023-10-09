pub(self) const SEARCH_URL: &str = "https://jisho.org/api/v1/search/words/?keyword=";

pub mod data;

use teloxide::{requests::Requester, types::Message, Bot};

use crate::HandlerResult;

use self::data::JishoReply;

pub(crate) async fn search_word(bot: Bot, request: Message) -> HandlerResult {
    let word_to_search = request.text().unwrap();
    let reply = reqwest::get(format!("{SEARCH_URL}{word_to_search}"))
        .await?
        .json::<JishoReply>()
        .await?;
    match reply.data {
        None => {
            bot.send_message(request.chat.id, "По вашему запросу ничего не найдено :(")
                .await?;
        }
        Some(_) => {
            bot.send_message(request.chat.id, get_formatted_reply(reply))
                .await?;
        }
    }
    Ok(())
}

fn get_formatted_reply(reply: JishoReply) -> String {
    let jap_words = acquire_jap(&reply);
    let jlpt_level = get_jlpt(&reply);
    let meanings = get_meanings(&reply);
    format!("Слова: \n{jap_words}\n\nУровень JLPT: \n{jlpt_level}\n\nЗначения: \n{meanings}")
}

fn acquire_jap(reply: &JishoReply) -> String {
    let jps = &reply.data.as_ref().unwrap().japanese;
    match jps.len() {
        0 => String::from("Отсутствуют"),
        _ => jps
            .iter()
            .map(|jp| jp.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
    }
}

fn get_jlpt(reply: &JishoReply) -> String {
    let levels = &reply.data.as_ref().unwrap().jlpt;
    match levels {
        None => String::from("Отсутствует"),
        Some(jlpt_level) => jlpt_level.clone(),
    }
}

fn get_meanings(reply: &JishoReply) -> String {
    let meanings = &reply.data.as_ref().unwrap().senses;
    match meanings.len() {
        0 => String::from("Отсутствуют"),
        _ => meanings
            .iter()
            .map(|sense| sense.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
    }
}
