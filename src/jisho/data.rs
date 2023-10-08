use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct JishoReply {
    pub(super) data: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
pub(super) struct Entry {
    pub(super) japanese: Vec<JapaneseMeaning>,
    pub(super) jlpt: Vec<String>,
    pub(super) senses: Vec<Sense>,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct JapaneseMeaning {
    pub(super) reading: Option<String>,
    pub(super) word: Option<String>,
}
impl Display for JapaneseMeaning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.word {
            Some(word) => match &self.reading {
                Some(reading) => write!(f, "- {}「{}」", word, reading),
                None => write!(f, "- {}", word),
            },
            None => match &self.reading {
                Some(reading) => write!(f, "- {}", reading),
                None => write!(f, ""),
            },
        }
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct Sense {
    pub(super) english_definitions: Vec<String>,
}

impl Display for Sense {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "・ {}", self.english_definitions.join("\n・ "))
    }
}
