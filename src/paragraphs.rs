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

    // TODO: Free tail recursion.
    #[inline(always)]
    fn inner_next(
        &mut self,
        indentation: usize,
        next_new_line_index: usize,
    ) -> Option<Paragraph<'a>> {
        let following_text = &self.text[next_new_line_index..];
        trace!(following_text, next_new_line_index);
        let mut ignore = false;

        // NB: Side effect blocks can be short-circuited.
        if following_text.is_empty()
            || following_text.starts_with('\n')
            || (!self.allow_indented_paragraphs
                && first_line_indentation(following_text) != indentation)
            || (self.next_is_ignore_paragraph && next_new_line_index > 0 && {
                ignore = true;
                self.next_is_ignore_paragraph = false;
                true
            })
            || (self.paragraph_starts.ignore_line_matches(following_text) && {
                self.next_is_ignore_paragraph = true;
                next_new_line_index != 0
            })
            || (self.next_is_single_paragraph && next_new_line_index > 0 && {
                trace!(next_is_single_paragraph = self.next_is_single_paragraph);
                self.next_is_single_paragraph = false;
                true
            })
            || (self.paragraph_starts.single_line_matches(following_text) && {
                self.next_is_single_paragraph = true;
                next_new_line_index != 0
            })
            || (next_new_line_index != 0
                && self.paragraph_starts.multi_line_matches(following_text))
        {
            let yielded = Paragraph {
                ignore,
                indentation,
                words: &self.text[..next_new_line_index],
            };
            self.text = match next_new_line_index {
                0 => &self.text[1..], // Yielded an empty paragraph.
                _ => following_text,
            };
            return Some(yielded);
        }

        let line_break_index = following_text
            .find('\n')
            .unwrap_or(following_text.len() - 1);
        self.inner_next(indentation, next_new_line_index + line_break_index + 1)
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
        self.inner_next(indentation, 0)
    }
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
