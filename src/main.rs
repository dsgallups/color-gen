use anyhow::Result;
use clap::Parser;
use color_gen::{Args, handle_args};

fn main() -> Result<()> {
    let args = Args::parse();
    handle_args(args)
}
