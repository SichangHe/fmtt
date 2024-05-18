use std::str::Chars;

use tracing::{debug, trace};

mod paragraphs;

pub use paragraphs::{Paragraph, ParagraphsIter};

pub fn format(text: &str, line_width: usize) -> Vec<&str> {
    let mut result = Vec::with_capacity(text.len() / 32);

    for paragraph in ParagraphsIter::new(text) {
        debug!(?paragraph);
        result.extend(paragraph.format(line_width));
    }

    result
}

#[derive(Copy, Clone, Debug, Default)]
pub struct SplitPoints {
    pub sub_start: SplitPoint,
    pub end: SplitPoint,
    pub sub_end: SplitPoint,
    pub connection_word: SplitPoint,
}

impl SplitPoints {
    /// Register chosen a split point with `n_char_after` characters after it.
    fn register_n_char_after(&mut self, n_char_after: usize) {
        for part in self.parts_ordered_mut() {
            if part.n_char_after >= n_char_after {
                // This split point was before the chosen one,
                // so it is now invalid.
                part.index = 0;
            } else {
                // This split point was after the chosen one,
                // so it is unaffected.
            }
        }
    }

    /// Signal that `reduction` number of splits have been consumed.
    fn reduce_index(&mut self, reduction: usize) {
        for part in self.parts_ordered_mut() {
            part.index = part.index.saturating_sub(reduction);
        }
    }

    pub fn parts_ordered_mut(&mut self) -> [&mut SplitPoint; 4] {
        [
            &mut self.end,
            &mut self.sub_end,
            &mut self.sub_start,
            &mut self.connection_word,
        ]
    }

    pub fn register_split(&mut self, split: &str, split_len: usize, n_split: usize) {
        for part in self.parts_ordered_mut() {
            part.n_char_after += split_len;
        }
        match word_sentence_position(split) {
            SentencePosition::End => {
                self.end.index = n_split;
                self.end.n_char_after = 0;
            }
            SentencePosition::SubEnd => {
                self.sub_end.index = n_split;
                self.sub_end.n_char_after = 0;
            }
            SentencePosition::SubStart => {
                self.sub_start.index = n_split.saturating_sub(1);
                self.sub_start.n_char_after = split_len;
            }
            SentencePosition::ConnectionWord => {
                self.connection_word.index = n_split;
                self.connection_word.n_char_after = 0;
            }
            SentencePosition::Other => {}
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default()
    }
}

impl Iterator for SplitPoints {
    type Item = SplitPoint;

    fn next(&mut self) -> Option<Self::Item> {
        let maybe_best_split_point = self
            .parts_ordered_mut()
            .into_iter()
            .filter(|split_point| split_point.index > 0)
            .map(|split_point| *split_point)
            .next();
        maybe_best_split_point.map(
            |split_point @ SplitPoint {
                 index,
                 n_char_after,
             }| {
                self.reduce_index(index);
                self.register_n_char_after(n_char_after);
                split_point
            },
        )
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct SplitPoint {
    pub index: usize,
    pub n_char_after: usize,
}

const MAX_ABBR_LEN: usize = 5;

/// Whether a word ends with a split point.
/// Handles abbreviations using heuristics.
fn word_sentence_position(word: &str) -> SentencePosition {
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

fn is_abbreviation(chars: &mut Chars) -> bool {
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

fn is_sentence_separator(char: char) -> bool {
    matches!(char, '.' | '!' | '?' | '…' | '。' | '，' | '？' | '！')
}

fn is_sub_sentence_separator(char: char) -> bool {
    matches!(
        char,
        ',' | ';' | ':' | '，' | '：' | '；' | ')' | '）' | '}' | '｝' | ']' | '］'
    )
}

fn is_sub_sentence_start(char: char) -> bool {
    matches!(
        char,
        '(' | '（' | '{' | '〖' | '『' | '｛' | '[' | '「' | '【' | '〔' | '［' | '〚' | '〘'
    )
}

fn is_connection_word(word: &str) -> bool {
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

pub fn get_indentation(line: &str) -> usize {
    for (index, char) in line.chars().enumerate() {
        if char != ' ' {
            return index;
        }
    }

    0
}

#[cfg(test)]
mod tests;
