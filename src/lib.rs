use std::str::Chars;

use regex::Regex;
use tracing::{debug, trace};

pub mod paragraphs;
pub mod split_points;
pub mod words;

use {paragraphs::*, split_points::*, words::*};

pub fn format<'a>(
    text: &'a str,
    line_width: usize,
    allow_indented_paragraphs: bool,
    paragraph_starts: &'a ParagraphStarts,
) -> Vec<&'a str> {
    let mut result = Vec::with_capacity(text.len() / 32);

    for paragraph in ParagraphsIter::new(text, allow_indented_paragraphs, paragraph_starts) {
        debug!(?paragraph);
        result.extend(paragraph.format(line_width));
    }

    result
}

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Default)]
pub struct ParagraphStarts {
    pub single_line: Option<Regex>,
    pub multi_line: Option<Regex>,
    // TODO: ignore_line: Regex,
}

const MARKDOWN_SINGLE_LINE_STARTS: [&str; 3] = ["#{1,6} ", r"---+[$\n]", r"===+[$\n]"];
const MARKDOWN_MULTI_LINE_STARTS: [&str; 2] = ["[-*] ", r"\d+\. "];

impl ParagraphStarts {
    pub fn single_line_matches(&self, text: &str) -> bool {
        self.single_line.as_ref().map(|re| re.is_match(text)) == Some(true) && {
            trace!("single_line match");
            true
        }
    }

    pub fn multi_line_matches(&self, text: &str) -> bool {
        self.multi_line.as_ref().map(|re| re.is_match(text)) == Some(true) && {
            trace!("multi_line match");
            true
        }
    }

    pub fn preset(markdown_friendly: bool, latex_friendly: bool) -> Result<Self, regex::Error> {
        let mut single_line = Vec::new();
        let mut multi_line = Vec::new();

        if markdown_friendly {
            single_line.extend(MARKDOWN_SINGLE_LINE_STARTS);
            multi_line.extend(MARKDOWN_MULTI_LINE_STARTS);
        }
        // TODO: if latex_friendly
        Self::try_from_str_slices(&single_line, &multi_line)
    }

    pub fn try_from_str_slices(
        single_line: &[&str],
        multi_line: &[&str],
    ) -> Result<Self, regex::Error> {
        let single_line = match single_line.is_empty() {
            true => None,
            false => Some(Regex::new(&format!("^(:?{})", &single_line.join("|")))?),
        };
        let multi_line = match multi_line.is_empty() {
            true => None,
            false => Some(Regex::new(&format!("^(:?{})", &multi_line.join("|")))?),
        };
        Ok(Self {
            single_line,
            multi_line,
        })
    }
}
