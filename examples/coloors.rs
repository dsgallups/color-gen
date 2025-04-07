use anyhow::Result;
use color_gen::*;

fn main() -> Result<()> {
    handle_args(Args::new(
        "examples/example.json",
        "example_output/coloors_output.rs",
    ))
}
