static SMALL_VOWEL: &str = "ゃャょョゅュぁァぃィぅゥぇェぉォ";
static N_ENDING: &str = "ん";

fn last(word: &str) -> &str {
    // Kana contains 3 bytes
    assert!(word.len() >= 3);
    word.get(word.len() - 3..word.len() - 1).unwrap()
}

pub(super) fn check_small_kana(word: &str) -> bool {
    SMALL_VOWEL.contains(last(word))
}

pub(super) fn check_n_ending(word: &str) -> bool {
    last(word) == N_ENDING
}

pub(super) fn check_for_stop(word: &str) -> bool {
    return word.to_lowercase().trim() == "shiritory stop";
}

pub(super) fn check_for_twice(word: &str) -> bool {
    todo!()
}

fn check_with_last(to_check: &str) -> bool {
    todo!()
}
