use std::str::Chars;

use regex::Regex;
use serde::{Deserialize, Serialize};
use tailcall::tailcall;
use tracing::{debug, trace};

pub mod paragraph_start;
pub mod paragraphs;
pub mod split_points;
pub mod words;

pub use crate::{paragraph_start::ParagraphStarts, paragraphs::Hanging};
use {paragraphs::*, split_points::*, words::*};

pub fn format<'a>(
    text: &'a str,
    line_width: usize,
    hanging_config: Hanging,
    paragraph_starts: &'a ParagraphStarts,
) -> Vec<&'a str> {
    let mut result = Vec::with_capacity(text.len() / 32);

    for paragraph in ParagraphsIter::new(text, hanging_config, paragraph_starts) {
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
    /// - `hanging_config` can be "disallow", "flatten", or "hang".
    ///
    /// See <https://github.com/SichangHe/fmtt> for the options.
    #[pyfunction]
    #[pyo3(signature = (
        text,
        line_width=80,
        hanging_config="disallow",
        single_line_starts=vec![],
        multi_line_starts=vec![],
        ignore_line_starts=vec![],
    ))]
    fn format(
        text: &str,
        line_width: usize,
        hanging_config: &str,
        single_line_starts: Vec<String>,
        multi_line_starts: Vec<String>,
        ignore_line_starts: Vec<String>,
    ) -> PyResult<String> {
        let hanging_config = serde_json::from_str(hanging_config)
            .map_err(|why| PyValueError::new_err(format!("{why}")))?;
        let paragraph_starts = ParagraphStarts::try_from_str_slices(
            &borrowed_str_slice(&single_line_starts),
            &borrowed_str_slice(&multi_line_starts),
            &borrowed_str_slice(&ignore_line_starts),
        )
        .map_err(|why| PyValueError::new_err(format!("{why}")))?;
        let formatted_words = super::format(text, line_width, hanging_config, &paragraph_starts);
        Ok(formatted_words.join(""))
    }

    fn borrowed_str_slice(slice: &[String]) -> Vec<&str> {
        slice.iter().map(String::as_str).collect()
    }
}

#[cfg(test)]
mod tests;
