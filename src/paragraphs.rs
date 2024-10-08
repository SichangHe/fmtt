use super::*;

pub struct ParagraphsIter<'a> {
    text: &'a str,
    hanging_config: Hanging,
    paragraph_starts: &'a ParagraphStarts,
    next_is_single_paragraph: bool,
    next_is_ignore_paragraph: bool,
}

/// Options to treat hanging paragraphs such as:
/// ```markdown
/// This paragraph has its
///     second line hanging.
/// ```
#[derive(clap::ValueEnum, Copy, Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Hanging {
    /// Disallow hanging. Any indentation change starts a new paragraph.
    #[default]
    Disallow,
    /// Ignore indentation changes; remove extra indentation in hanging lines.
    Flatten,
    /// Allow the second line to start hanging (having more indentation);
    /// keep the hanging lines as is.
    Hang,
}

impl<'a> ParagraphsIter<'a> {
    pub fn new(
        text: &'a str,
        hanging_config: Hanging,
        paragraph_starts: &'a ParagraphStarts,
    ) -> Self {
        trace!(?hanging_config, ?paragraph_starts);
        Self {
            text,
            hanging_config,
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
    let mut hanging_indentation = None;

    // NB: Side effect blocks can be short-circuited.
    if following_text.is_empty()
        || following_text.starts_with('\n')
        || match (first_line_indentation(following_text), iter.hanging_config) {
            (hanging, Hanging::Hang) if hanging > indentation => {
                match iter.text[..next_new_line_index].rfind('\n') {
                    // Only the second line can be hanging, but this is not.
                    Some(_) => true,
                    None => {
                        hanging_indentation = Some(hanging);
                        false
                    }
                }
            }
            (hanging, Hanging::Disallow | Hanging::Hang) if hanging != indentation => true,
            _ => false,
        }
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
        let config = ParagraphConfig {
            ignore,
            indentation,
            hanging_indentation,
        };
        let yielded = Paragraph {
            config,
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
    pub config: ParagraphConfig,
    pub words: &'a str,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ParagraphConfig {
    pub ignore: bool,
    pub indentation: usize,
    pub hanging_indentation: Option<usize>,
}

const SPACES: &str =
    "                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                ";

impl<'a> Paragraph<'a> {
    pub fn format(&self, line_width: usize) -> Vec<&'a str> {
        if self.config.ignore {
            return vec![self.words];
        } else if self.words.is_empty() {
            return vec!["\n"];
        }
        let mut result = Vec::with_capacity(self.words.len() / 32);
        paragraph_inner_format(
            &mut self.config.clone(),
            &mut (line_width + 1 - self.config.indentation),
            &mut result,
            &mut SplitPoints::default(),
            &mut Vec::with_capacity(line_width / 2),
            &mut 0,
            &mut 0,
            &mut self.words.split_ascii_whitespace(),
            &mut 0,
        );
        result
    }
}

/// Internal function used to format a paragraph.
#[inline(always)]
#[allow(unreachable_code, clippy::too_many_arguments)]
#[tailcall]
pub fn paragraph_inner_format<'a, I>(
    config: &mut ParagraphConfig,
    available_line_width: &mut usize,
    result: &mut Vec<&'a str>,
    split_points: &mut SplitPoints,
    to_be_split: &mut Vec<&'a str>,
    n_char: &mut usize,
    split_len: &mut usize,
    splits: &mut I,
    drain_index: &mut usize,
) where
    I: Iterator<Item = &'a str>,
{
    trace!(n_char, split_len, drain_index, ?split_points, ?to_be_split);

    if *drain_index > 0 {
        result.push(&SPACES[..config.indentation]);
        for word in to_be_split.drain(..*drain_index) {
            result.push(word);
            result.push(" ");
        }
        debug!("Last word in line: {:?}.", result.last());
        *result.last_mut().expect("We just pushed") = "\n";
        *drain_index = 0;
        // Handle hanging:
        if let Some(hanging_indentation) = config.hanging_indentation.take() {
            *available_line_width -= hanging_indentation - config.indentation;
            config.indentation = hanging_indentation;
        }
    } else if n_char < available_line_width || to_be_split.len() <= 1 {
        if let Some(&split) = to_be_split.last() {
            split_points.register_split(split, *split_len, to_be_split.len());
        }
        if let Some(split) = splits.next() {
            *split_len = split.chars().count() + 1;
            to_be_split.push(split);
            *n_char += *split_len;
        } else {
            if to_be_split.is_empty() {
                return;
            }
            *drain_index = to_be_split.len();
        };
    } else {
        match (split_len >= available_line_width, split_points.next()) {
            (true, _) | (_, None) => {
                // Either the new split is too longer,
                // or no valid split point was found.
                // Drain the entire buffer once.
                *drain_index = to_be_split.len().saturating_sub(1);
                split_points.reset();
                *n_char = *split_len;
            }
            (
                _,
                Some(SplitPoint {
                    index,
                    n_char_after,
                }),
            ) => {
                *drain_index = index;
                *n_char = n_char_after + *split_len;
            }
        }
    }

    paragraph_inner_format(
        config,
        available_line_width,
        result,
        split_points,
        to_be_split,
        n_char,
        split_len,
        splits,
        drain_index,
    )
}
