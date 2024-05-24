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

#[cfg(feature = "py")]
#[pyo3::pymodule]
mod _lowlevel {
    use pyo3::{exceptions::PyValueError, prelude::*};

    use super::*;

    /// Format text diff-friendly by breaking lines of sensible punctuations and
    /// words.
    ///
    /// See <https://github.com/SichangHe/fmtt> for the options.
    #[pyfunction]
    #[pyo3(signature = (
        text,
        line_width=80,
        allow_indented_paragraphs=false,
        single_line_starts=vec![],
        multi_line_starts=vec![],
        ignore_line_starts=vec![],
    ))]
    fn format(
        text: &str,
        line_width: usize,
        allow_indented_paragraphs: bool,
        single_line_starts: Vec<String>,
        multi_line_starts: Vec<String>,
        ignore_line_starts: Vec<String>,
    ) -> PyResult<String> {
        let paragraph_starts = ParagraphStarts::try_from_str_slices(
            &borrowed_str_slice(&single_line_starts),
            &borrowed_str_slice(&multi_line_starts),
            &borrowed_str_slice(&ignore_line_starts),
        )
        .map_err(|why| PyValueError::new_err(format!("{why}")))?;
        let formatted_words = super::format(
            text,
            line_width,
            allow_indented_paragraphs,
            &paragraph_starts,
        );
        Ok(formatted_words.join(""))
    }

    fn borrowed_str_slice(slice: &[String]) -> Vec<&str> {
        slice.iter().map(String::as_str).collect()
    }
}

#[cfg(test)]
mod tests;
