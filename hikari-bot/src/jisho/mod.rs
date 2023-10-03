pub(self) const SEARCH_URL: &str = "https://jisho.org/api/v1/search/words/?keyword=";

pub mod data;

use self::data::{JapaneseMeaning, JishoReply};

pub(crate) async fn search_word(request: String) -> Result<String, Box<dyn std::error::Error>> {
    let reply = reqwest::get(format!("{SEARCH_URL}{request}"))
        .await?
        .json::<JishoReply>()
        .await?;
    match reply.data.len() {
        0 => Ok(String::from("По вашему запросу ничего не найдено :(")),
        _ => Ok(get_formatted_reply(reply)),
    }
}

fn get_formatted_reply(reply: JishoReply) -> String {
    let jap_words = acquire_jap(&reply);
    let jlpt_level = get_jlpt(&reply);
    let meanings = get_meanings(&reply);
    format!("Слова: \n{jap_words}\n\nУровень JLPT: \n{jlpt_level}\n\nЗначения: \n{meanings}")
}

fn acquire_jap(reply: &JishoReply) -> String {
    let jps = &reply.data[0].japanese;
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
    let levels = &reply.data[0].jlpt;
    match levels.len() {
        0 => String::from("Отсутствует"),
        _ => levels.join("\n"),
    }
}

fn get_meanings(reply: &JishoReply) -> String {
    let meanings = &reply.data[0].senses;
    match meanings.len() {
        0 => String::from("Отсутствуют"),
        _ => meanings
            .iter()
            .map(|sense| sense.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
    }
}
