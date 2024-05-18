use std::str::Chars;

use tracing::{debug, trace};

pub fn format(text: &str, line_width: usize) -> Vec<&str> {
    let mut result = Vec::with_capacity(text.len() / 32);

    for paragraph in ParagraphsIter::new(text) {
        debug!(?paragraph);
        result.extend(paragraph.format(line_width));
    }

    result
}

struct ParagraphsIter<'a> {
    text: &'a str,
}

impl<'a> ParagraphsIter<'a> {
    fn new(text: &'a str) -> Self {
        Self { text }
    }
}

impl<'a> Iterator for ParagraphsIter<'a> {
    type Item = Paragraph<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let trimmed_start_new_lines = self.text.trim_start_matches('\n');
        if trimmed_start_new_lines.len() != self.text.len() {
            self.text = trimmed_start_new_lines;
            return Some(Paragraph {
                indentation: 0,
                words: "",
            });
        }

        if self.text.is_empty() {
            return None;
        }

        let mut following_text = self.text;
        let indentation = get_indentation(self.text);
        let mut new_line_index_relative_to_original = 0;
        loop {
            let new_line_index = match following_text.find('\n') {
                Some(i) => i,
                None => {
                    let yielded = Some(Paragraph {
                        indentation,
                        words: self.text,
                    });
                    self.text = "";
                    return yielded;
                }
            } + 1;
            new_line_index_relative_to_original += new_line_index;

            following_text = &following_text[new_line_index..];
            if following_text.starts_with('\n') || get_indentation(following_text) != indentation {
                let yielded = Some(Paragraph {
                    indentation,
                    words: &self.text[..new_line_index_relative_to_original],
                });
                self.text = following_text;
                return yielded;
            }
        }
    }
}

#[derive(Debug)]
pub struct Paragraph<'a> {
    indentation: usize,
    words: &'a str,
}

pub const SPACES: &str =
    "                                                                                                                                                                                                                                                                ";

impl<'a> Paragraph<'a> {
    pub fn format(&self, line_width: usize) -> Vec<&'a str> {
        if self.words.is_empty() {
            return vec!["\n"];
        }

        let mut result = Vec::with_capacity(self.words.len() / 32);

        self.inner_format(line_width, &mut result);

        result
    }

    #[inline(always)]
    fn inner_format(&self, line_width: usize, result: &mut Vec<&'a str>) {
        let line_width = line_width + 1 - self.indentation;

        let mut split_points = SplitPoints::default();
        let mut to_be_split = Vec::with_capacity(line_width / 2);
        let mut n_char = 0;

        macro_rules! push_line {
            ($pushed_words:expr) => {
                result.push(&SPACES[..self.indentation]);
                for word in $pushed_words {
                    result.push(word);
                    result.push(" ");
                }
                result.pop();

                debug!("Last word in line: {:?}.", result.last());
                result.push("\n");
            };
        }

        for split in self.words.split_whitespace() {
            let split_len = split.chars().count() + 1;

            to_be_split.push(split);
            n_char += split_len;
            trace!(split, n_char, ?split_points, ?to_be_split);

            for _try in 0usize..2 {
                if n_char <= line_width || to_be_split.is_empty() {
                    break;
                }
                match split_points.next() {
                    None => {
                        let last_index = to_be_split.len().saturating_sub(1);
                        push_line!(to_be_split.drain(..last_index));
                        split_points.reset();
                        n_char = split_len;
                    }
                    Some(
                        split_point @ SplitPoint {
                            index,
                            n_char_after,
                        },
                    ) => {
                        trace!(?split_point, "next");
                        push_line!(to_be_split.drain(..index));
                        n_char = n_char_after + split_len;
                    }
                }
            }

            split_points.register_split(split, split_len, to_be_split.len());
        }

        if !to_be_split.is_empty() {
            push_line!(to_be_split.drain(..));
        }
    }
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
