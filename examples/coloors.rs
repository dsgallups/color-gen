use anyhow::Result;
use color_gen::*;

fn main() -> Result<()> {
    handle_args(Args::new(
        "examples/example.json",
        "examples/coloors_output.rs",
    ))
}
