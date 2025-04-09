use anyhow::Result;
use cargo_color_gen::*;

fn main() -> Result<()> {
    handle_args(Args::new(
        "examples/example.json",
        "example_output/coolors_output.rs",
    ))
}
