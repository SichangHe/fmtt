use super::*;

/// Regex's for determining special paragraph starts.
#[derive(Clone, Debug, Default)]
pub struct ParagraphStarts {
    pub single_line: Option<Regex>,
    pub multi_line: Option<Regex>,
    pub ignore_line: Option<Regex>,
}

const MARKDOWN_SINGLE_LINE_STARTS: [&str; 3] = ["#{1,6} ", r"---+[$\n]", r"===+[$\n]"];
const MARKDOWN_MULTI_LINE_STARTS: [&str; 2] = ["[-*] ", r"\d+\. "];
const LATEX_MULTI_LINE_STARTS: [&str; 1] = [r"\\"];
const LATEX_IGNORE_LINE_STARTS: [&str; 1] = ["%"];

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

    pub fn ignore_line_matches(&self, text: &str) -> bool {
        self.ignore_line.as_ref().map(|re| re.is_match(text)) == Some(true) && {
            trace!("ignore_line match");
            true
        }
    }

    /// Generate using configuration presets.
    pub fn preset(markdown_friendly: bool, latex_friendly: bool) -> Result<Self, regex::Error> {
        let mut single_line = Vec::new();
        let mut multi_line = Vec::new();
        let mut ignore_line = Vec::new();

        if markdown_friendly {
            single_line.extend(MARKDOWN_SINGLE_LINE_STARTS);
            multi_line.extend(MARKDOWN_MULTI_LINE_STARTS);
        }
        if latex_friendly {
            multi_line.extend(LATEX_MULTI_LINE_STARTS);
            ignore_line.extend(LATEX_IGNORE_LINE_STARTS);
        }
        Self::try_from_str_slices(&single_line, &multi_line, &ignore_line)
    }

    pub fn try_from_str_slices(
        single_line: &[&str],
        multi_line: &[&str],
        ignore_line: &[&str],
    ) -> Result<Self, regex::Error> {
        let single_line = (!single_line.is_empty()).then_some(or_regex_from_strs(single_line)?);
        let multi_line = (!multi_line.is_empty()).then_some(or_regex_from_strs(multi_line)?);
        let ignore_line = (!ignore_line.is_empty()).then_some(or_regex_from_strs(ignore_line)?);
        Ok(Self {
            single_line,
            multi_line,
            ignore_line,
        })
    }
}

/// Leading spaces are allowed.
fn or_regex_from_strs(slice: &[&str]) -> Result<Regex, regex::Error> {
    Regex::new(&format!(r"^ *(:?{})", &slice.join("|")))
}
