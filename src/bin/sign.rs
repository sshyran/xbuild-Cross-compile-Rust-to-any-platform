use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    input: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    xstore::sign::android::read_zip(&args.input)?;
    Ok(())
}