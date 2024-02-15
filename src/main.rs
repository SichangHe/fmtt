use std::io::*;

use anyhow::Result;

use fmtt::*;
use tracing_subscriber::EnvFilter;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mut input = Vec::with_capacity(4096);
    let mut std_in = BufReader::new(stdin());
    let mut std_out = BufWriter::new(stdout());

    std_in.read_to_end(&mut input)?;
    let input = String::from_utf8(input)?;

    let formatted = format(&input);
    for blob in formatted {
        std_out.write_all(blob.as_bytes())?;
    }
    std_out.flush()?;

    Ok(())
}
