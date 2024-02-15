use std::{fs::File, io::*, path::PathBuf};

use anyhow::Result;
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

    let formatted = format(&input, app.line_width);

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
    long_about = "A stupid text formatter that tries to break lines on sensible punctuations."
)]
struct App {
    #[arg(short, long, default_value = "80")]
    line_width: usize,

    #[arg(short, long)]
    filename: Option<PathBuf>,

    #[arg(short, long, default_value = "false")]
    change_in_place: bool,
}
