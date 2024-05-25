use std::str::SplitWhitespace;

use super::*;

pub struct ParagraphsIter<'a> {
    text: &'a str,
    allow_indented_paragraphs: bool,
    paragraph_starts: &'a ParagraphStarts,
    next_is_single_paragraph: bool,
    next_is_ignore_paragraph: bool,
}

impl<'a> ParagraphsIter<'a> {
    pub fn new(
        text: &'a str,
        allow_indented_paragraphs: bool,
        paragraph_starts: &'a ParagraphStarts,
    ) -> Self {
        trace!(allow_indented_paragraphs, ?paragraph_starts);
        Self {
            text,
            allow_indented_paragraphs,
            paragraph_starts,
            next_is_single_paragraph: false,
            next_is_ignore_paragraph: false,
        }
    }

    /// Compress multiple starting line breaks into a single, if applicable.
    #[inline(always)]
    fn trim_extra_start_line_breaks(&mut self) {
        for (index, char) in self.text.chars().enumerate() {
            match char {
                '\n' => {}
                _ => {
                    let index = index.saturating_sub(1);
                    self.text = &self.text[index..];
                    break;
                }
            }
        }
    }
}

impl<'a> Iterator for ParagraphsIter<'a> {
    type Item = Paragraph<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.text.is_empty() {
            return None;
        }
        self.trim_extra_start_line_breaks();
        let indentation = first_line_indentation(self.text);
        iter_inner_next(self, indentation, 0)
    }
}

// This has to be a static function for `tailcall`.
#[inline(always)]
#[allow(unreachable_code)]
#[tailcall]
fn iter_inner_next<'a>(
    iter: &mut ParagraphsIter<'a>,
    indentation: usize,
    next_new_line_index: usize,
) -> Option<Paragraph<'a>> {
    let following_text = &iter.text[next_new_line_index..];
    trace!(following_text, next_new_line_index);
    let mut ignore = false;

    // NB: Side effect blocks can be short-circuited.
    if following_text.is_empty()
        || following_text.starts_with('\n')
        || (!iter.allow_indented_paragraphs
            && first_line_indentation(following_text) != indentation)
        || (iter.next_is_ignore_paragraph && next_new_line_index > 0 && {
            ignore = true;
            iter.next_is_ignore_paragraph = false;
            true
        })
        || (iter.paragraph_starts.ignore_line_matches(following_text) && {
            iter.next_is_ignore_paragraph = true;
            next_new_line_index != 0
        })
        || (iter.next_is_single_paragraph && next_new_line_index > 0 && {
            trace!(next_is_single_paragraph = iter.next_is_single_paragraph);
            iter.next_is_single_paragraph = false;
            true
        })
        || (iter.paragraph_starts.single_line_matches(following_text) && {
            iter.next_is_single_paragraph = true;
            next_new_line_index != 0
        })
        || (next_new_line_index != 0 && iter.paragraph_starts.multi_line_matches(following_text))
    {
        let yielded = Paragraph {
            ignore,
            indentation,
            words: &iter.text[..next_new_line_index],
        };
        iter.text = match next_new_line_index {
            0 => &iter.text[1..], // Yielded an empty paragraph.
            _ => following_text,
        };
        return Some(yielded);
    }

    let line_break_index = following_text
        .find('\n')
        .unwrap_or(following_text.len() - 1);
    iter_inner_next(
        iter,
        indentation,
        next_new_line_index + line_break_index + 1,
    )
}

pub fn first_line_indentation(line: &str) -> usize {
    for (index, char) in line.chars().enumerate() {
        match char {
            ' ' => {}
            '\n' => return 0,
            _ => return index,
        }
    }

    0
}

#[derive(Clone, Debug)]
pub struct Paragraph<'a> {
    pub ignore: bool,
    pub indentation: usize,
    pub words: &'a str,
}

const SPACES: &str =
    "                                                                                                                                                                                                                                                                ";

impl<'a> Paragraph<'a> {
    pub fn format(&self, line_width: usize) -> Vec<&'a str> {
        if self.ignore {
            return vec![self.words];
        } else if self.words.is_empty() {
            return vec!["\n"];
        }
        paragraph_inner_format(
            self.indentation,
            line_width + 1 - self.indentation,
            Vec::with_capacity(self.words.len() / 32),
            SplitPoints::default(),
            Vec::with_capacity(line_width / 2),
            0,
            0,
            self.words.split_whitespace(),
        )
    }
}

#[inline(always)]
#[allow(unreachable_code, clippy::too_many_arguments)]
#[tailcall]
fn paragraph_inner_format<'a>(
    indentation: usize,
    available_line_width: usize,
    mut result: Vec<&'a str>,
    mut split_points: SplitPoints,
    mut to_be_split: Vec<&'a str>,
    mut n_char: usize,
    mut split_len: usize,
    mut splits: SplitWhitespace<'a>,
) -> Vec<&'a str> {
    trace!(n_char, split_len, ?split_points, ?to_be_split);
    macro_rules! push_line {
        ($pushed_words:expr) => {
            result.push(&SPACES[..indentation]);
            for word in $pushed_words {
                result.push(word);
                result.push(" ");
            }
            result.pop();

            debug!("Last word in line: {:?}.", result.last());
            result.push("\n");
        };
    }

    if n_char < available_line_width || to_be_split.len() <= 1 {
        if let Some(&split) = to_be_split.last() {
            split_points.register_split(split, split_len, to_be_split.len());
        }
        let Some(split) = splits.next() else {
            if !to_be_split.is_empty() {
                push_line!(to_be_split.drain(..));
            }
            return result;
        };
        split_len = split.chars().count() + 1;
        to_be_split.push(split);
        n_char += split_len;
    } else {
        match (split_len >= available_line_width, split_points.next()) {
            (true, _) | (_, None) => {
                // Either the new split is too longer,
                // or no valid split point was found.
                // Drain the entire buffer once.
                let last_index = to_be_split.len().saturating_sub(1);
                push_line!(to_be_split.drain(..last_index));
                split_points.reset();
                n_char = split_len;
            }
            (
                _,
                Some(
                    split_point @ SplitPoint {
                        index,
                        n_char_after,
                    },
                ),
            ) => {
                trace!(?split_point, "next");
                push_line!(to_be_split.drain(..index));
                n_char = n_char_after + split_len;
            }
        }
    }

    paragraph_inner_format(
        indentation,
        available_line_width,
        result,
        split_points,
        to_be_split,
        n_char,
        split_len,
        splits,
    )
}
