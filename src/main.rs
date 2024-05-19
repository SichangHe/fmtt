use std::{fs::File, io::*, path::PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use fmtt::*;
use tracing_subscriber::EnvFilter;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app = App::parse();
    let input = if let Some(filename) = &app.filename {
        read_all(File::open(filename)?)?
    } else {
        read_all(stdin())?
    };

    let paragraph_starts = app.paragraph_starts()?;
    let formatted = format(
        &input,
        app.line_width,
        app.allow_indented_paragraphs,
        &paragraph_starts,
    );

    if let (true, Some(filename)) = (app.change_in_place, &app.filename) {
        write_all(File::create(filename)?, &formatted)?;
    } else {
        write_all(stdout(), &formatted)?;
    }

    Ok(())
}

fn read_all(from: impl Read) -> Result<String> {
    let mut input = Vec::with_capacity(4096);
    let mut from = BufReader::new(from);
    from.read_to_end(&mut input)?;
    Ok(String::from_utf8(input)?)
}

fn write_all(to: impl Write, formatted: &[&str]) -> Result<()> {
    let mut to = BufWriter::new(to);
    for blob in formatted {
        to.write_all(blob.as_bytes())?;
    }
    to.flush()?;

    Ok(())
}

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = r#"
ForMaT Text diff-friendly,
breaking lines on sensible punctuations and words to fit a line width.

Like fmt, FMTT is a text formatter;
it formats its input to have lines shorter than the line width limit
(if possible).
It reads an input file or StdIn and prints the formatted text to StdOut.
Like LaTeX,
FMTT does not distinguish different whitespaces or their amount except for
double line breaks; it only preserves leading spaces, not tabs.

This help message is formatted using FMTT itself as an example.
"#
)]
struct App {
    #[arg(
        short = 'w',
        long,
        default_value = "80",
        help = "Maximum line width limit."
    )]
    line_width: usize,

    #[arg(short, long, help = "Name of input file; if omitted, read from StdIn.")]
    filename: Option<PathBuf>,

    #[arg(
        short,
        long,
        default_value = "false",
        help = "If input file is provided, write output to it."
    )]
    change_in_place: bool,

    #[arg(
        short = 'p',
        long,
        default_value = "false",
        help = r#"Allow indented paragraphs.
If not set, any change indentation changes start a new paragraph."#
    )]
    allow_indented_paragraphs: bool,

    #[arg(
        short,
        long,
        default_value = "false",
        help = r#"Treat `# `/`## `/…/`###### `/`---`/`===`-started lines as single paragraphs;
treat `- `/`* `/regex`\d+\. `-started lines as paragraph starts.
Useful for Markdown, especially with `-p`."#
    )]
    markdown_friendly: bool,

    #[arg(
        short,
        long,
        default_value = "false",
        help = r#"Ignore `%`-started lines;
treat `\` started lines as paragraph starts.
Useful for LaTeX."#
    )]
    latex_friendly: bool,
}

impl App {
    fn paragraph_starts(&self) -> Result<ParagraphStarts> {
        ParagraphStarts::preset(self.markdown_friendly, self.latex_friendly)
            .context("Failed to build special paragraph starts handler.")
    }
}
