use std::str::Chars;

use tracing::{debug, trace};

pub mod paragraphs;
pub mod split_points;
pub mod words;

use {paragraphs::*, split_points::*, words::*};

pub fn format(text: &str, line_width: usize, allow_indented_paragraphs: bool) -> Vec<&str> {
    let mut result = Vec::with_capacity(text.len() / 32);

    for paragraph in ParagraphsIter::new(text, allow_indented_paragraphs) {
        debug!(?paragraph);
        result.extend(paragraph.format(line_width));
    }

    result
}

#[cfg(test)]
mod tests;
