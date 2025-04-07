use anyhow::Result;
use std::{
    fs::File,
    intrinsics::typed_swap_nonoverlapping,
    io::{self, Read, Write},
};
mod tailwind;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    input_file: Option<String>,

    #[arg(short, long)]
    output_file: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut input = String::new();
    match args.input_file {
        Some(file) => {
            let mut file = File::open(file).unwrap();
            file.read_to_string(&mut input)?;
            file.flush()?;
        }
        None => loop {
            input.clear();
            println!("Paste your JSON [press ENTER to continue]:");
            if io::stdin().read_line(&mut input).is_err() {
                println!("Failed to read input.");
                continue;
            }
            break;
        },
    };
    let result = generate(input)?;

    if let Some(output_path) = args.output_file {
        let Ok(mut of) = File::create(output_path) else {
            println!("Error creating file at path. Generated Output:\n=====\n{result}\n=====");
            return Ok(());
        };

        if let Err(e) = of.write_all(result.as_bytes()) {
            println!(
                "Error at path:\n{:?}\nGenerated Output:\n=====\n{result}\n=====",
                e
            );
            return Ok(());
        };
        println!("Result written to output.");

        return Ok(());
    }
    println!("Output\n=====\n{result}\n=====");
    Ok(())
}

pub fn generate(input: String) -> Result<String> {
    let val = serde_json::from_str(input.trim())?;
    todo!()
}
