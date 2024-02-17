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

        let mut split_point = 0;
        let mut to_be_split = Vec::with_capacity(line_width / 2);
        let mut n_char = 0;
        let mut n_char_after_split_point = 0;

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

            n_char += split_len;
            n_char_after_split_point += split_len;
            trace!(n_char, n_char_after_split_point, split, split_point);

            while n_char > line_width && !to_be_split.is_empty() {
                match split_point {
                    0 => {
                        push_line!(to_be_split.drain(..));
                        n_char_after_split_point = split_len;
                    }
                    _ => {
                        push_line!(to_be_split.drain(..split_point));
                        split_point = 0;
                    }
                }
                n_char = n_char_after_split_point;
            }

            to_be_split.push(split);
            if is_split_point_word(split) {
                split_point = to_be_split.len();
                n_char_after_split_point = 0;
            }
        }

        if !to_be_split.is_empty() {
            push_line!(to_be_split.drain(..));
        }
    }
}

/// Whether a word ends with a split point.
/// Handles abbreviations using heuristics.
fn is_split_point_word(word: &str) -> bool {
    let mut chars = word.chars();
    match chars.next_back() {
        Some('.') => {
            // Ends with a `.` and starts with an uppercase character.
            match chars.next() {
                Some(first_char) if first_char.is_uppercase() => {
                    // Avoid abbreviations.
                    let mut need_capital = false;
                    for char in chars {
                        match (need_capital, char) {
                            // `..`
                            (true, '.') => return true,
                            (_, '.') => need_capital = true,
                            (true, char) if char.is_uppercase() => need_capital = false,
                            // Non-capital letters following `.`
                            (true, _) => return true,
                            (false, char) if char.is_alphabetic() => {}
                            (false, _) => return true,
                        }
                    }

                    false
                }
                _ => true,
            }
        }
        Some(last_char) if is_sub_sentence_separator(last_char) => true,
        _ => false,
    }
}

fn is_sub_sentence_separator(char: char) -> bool {
    matches!(
        char,
        ',' | '.'
            | '!'
            | '?'
            | ';'
            | '…'
            | ':'
            | '。'
            | '，'
            | '？'
            | '！'
            | '：'
            | '；'
            | ')'
            | '）'
            | '}'
            | '｝'
            | ']'
            | '］'
    )
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
