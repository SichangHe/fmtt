use std::str::Chars;

use regex::Regex;
use tracing::{debug, trace};

pub mod paragraph_start;
pub mod paragraphs;
pub mod split_points;
pub mod words;

pub use paragraph_start::ParagraphStarts;
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
