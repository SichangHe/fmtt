use super::*;

pub struct ParagraphsIter<'a> {
    text: &'a str,
}

impl<'a> ParagraphsIter<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    fn maybe_yield_start_new_lines(&mut self) -> Option<Paragraph<'static>> {
        let trimmed_start_new_lines = self.text.trim_start_matches('\n');
        if trimmed_start_new_lines.len() != self.text.len() {
            self.text = trimmed_start_new_lines;
            Some(Paragraph {
                indentation: 0,
                words: "",
            })
        } else {
            None
        }
    }

    fn inner_next(&mut self) -> Option<Paragraph<'a>> {
        let mut following_text = self.text;
        let indentation = first_line_indentation(self.text);
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
            if following_text.starts_with('\n')
                || first_line_indentation(following_text) != indentation
            {
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

impl<'a> Iterator for ParagraphsIter<'a> {
    type Item = Paragraph<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let paragraph @ Some(_) = self.maybe_yield_start_new_lines() {
            return paragraph;
        }

        if self.text.is_empty() {
            return None;
        }

        self.inner_next()
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
    pub indentation: usize,
    pub words: &'a str,
}

const SPACES: &str =
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
