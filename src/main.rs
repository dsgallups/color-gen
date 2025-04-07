use anyhow::Result;
use cargo_color::{Args, handle_args};
use clap::Parser;

fn main() -> Result<()> {
    let args = Args::parse();
    handle_args(args)
}
