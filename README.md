# Cargo Color
Generate constant color palettes (for bevy via color schemes)

## Get Started
install via `cargo install cargo-color-gen`

Then, with a tailwind scheme and output file:
`cargo color-gen -i examples/example.json -o example_output/coloors_output.rs`

## Input Schema
A schema that looks something like
```json
{
    "sandy_brown": {
        "100": "#462001",
        "600": "#fdbc87",
        "900": "#feeee1"
    },
    "redwood": {
        "700": "#d58a8792", // alpha channel included
        "800": "#e3b1af",
        "900": "#f1d8d7",
        "NO_WORK": "#GGGGGG" // generated file will state this has failed
    },
```
should work.


## Help needed!

Please let me know what formats should be supported. Additionally, I've got some thoughts:

- it could be nice to make strongly typed palettes with enumerations
-
