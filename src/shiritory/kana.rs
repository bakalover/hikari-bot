use super::check;

pub(super) fn get_start_kana(previous_word: &str) -> String {
    match check::check_small_kana(previous_word) {
        true => previous_word
            .get(previous_word.len() - 7..previous_word.len() - 1)
            .unwrap()
            .to_string(),

        false => previous_word
            .get(previous_word.len() - 4..previous_word.len() - 1)
            .unwrap()
            .to_string(),
    }
}