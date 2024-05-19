use super::*;

/// Whether a word ends with a split point.
/// Handles abbreviations using heuristics.
pub fn word_sentence_position(word: &str) -> SentencePosition {
    use SentencePosition::*;
    match word.chars().next() {
        Some(first_char) if is_sub_sentence_start(first_char) => return SubStart,
        _ => {}
    };
    let mut chars = word.chars();
    match chars.next_back() {
        Some('.') if is_abbreviation(&mut chars) => {}
        Some(last_char) if is_sentence_separator(last_char) => return End,
        Some(last_char) if is_sub_sentence_separator(last_char) => return SubEnd,
        _ => {}
    }
    match is_connection_word(word) {
        true => ConnectionWord,
        false => Other,
    }
}

const MAX_ABBR_LEN: usize = 5;

pub fn is_abbreviation(chars: &mut Chars) -> bool {
    match chars.next() {
        Some(first_char) if first_char.is_uppercase() => {
            // Starts with an uppercase character.
            let mut word_len = 1;
            // The rest of the characters, starting with the 2nd one.
            for char in chars {
                match (word_len, char) {
                    // `..`
                    (0, '.') => return false,
                    (_, '.') => word_len = 0,
                    (0, char) if char.is_uppercase() => word_len = 1,
                    // Non-capital letters following `.`
                    (0, _) => return false,
                    (_, char) if word_len < MAX_ABBR_LEN && char.is_lowercase() => word_len += 1,
                    (_, _) => return false,
                }
            }

            true
        }
        // Does not start with an uppercase character.
        _ => false,
    }
}

pub fn is_sentence_separator(char: char) -> bool {
    matches!(char, '.' | '!' | '?' | '…' | '。' | '，' | '？' | '！')
}

pub fn is_sub_sentence_separator(char: char) -> bool {
    matches!(
        char,
        ',' | ';' | ':' | '，' | '：' | '；' | ')' | '）' | '}' | '｝' | ']' | '］'
    )
}

pub fn is_sub_sentence_start(char: char) -> bool {
    matches!(
        char,
        '(' | '（'
            | '{'
            | '〖'
            | '『'
            | '｛'
            | '['
            | '「'
            | '【'
            | '〔'
            | '［'
            | '〚'
            | '〘'
            | '@'
            | '#'
            | '$'
            | '%'
    )
}

pub fn is_connection_word(word: &str) -> bool {
    matches!(
        word,
        "and"
            | "or"
            | "but"
            | "except"
            | "that"
            | "which"
            | "who"
            | "where"
            | "when"
            | "while"
            | "though"
            | "although"
            | "in"
            | "on"
            | "of"
            | "by"
            | "for"
            | "from"
            | "to"
            | "through"
            | "with"
            | "via"
    )
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SentencePosition {
    /// Start of a sub-sentence.
    SubStart,
    /// End of a sentence.
    End,
    /// Start of a sub-sentence.
    SubEnd,
    /// Word to connect different parts of a sentence.
    ConnectionWord,
    /// Not a special sentence position.
    Other,
}

impl Default for SentencePosition {
    fn default() -> Self {
        Self::Other
    }
}
