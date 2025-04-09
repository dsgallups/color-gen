use cargo_color_gen::generate;

fn main() {
    let readme_json = r##"
    {
        "sandy_brown": {
            "100": "#462001",
            "600": "#fdbc87",
            "900": "#feeee1"
        },
        "redwood": {
            "700": "#d58a8792",
            "800": "#e3b1af",
            "900": "#f1d8d7",
            "NO_WORK": "#GGGGGG"
        }
    }
    "##;

    println!("Generated code: \n{}\n", generate(readme_json).unwrap());
}
