use anyhow::Result;
use cargo_color::*;

fn main() -> Result<()> {
    handle_args(Args::new(
        "examples/example.json",
        "example_output/coloors_output.rs",
    ))
}
