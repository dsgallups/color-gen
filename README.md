# Cargo Color
Generate constant color palettes (for bevy via color schemes)

## Get Started
install via `cargo install cargo-color-gen`

Then, with a tailwind scheme and output file:
`cargo color-gen -i examples/example.json -o example_output/coloors_output.rs`

### Input Schema
A schema that looks something like this.
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
    }
}
```
Additionally, it can be a JS object and this cli will attempt to reformat it as a JSON.

You can specify a JSON file with the `-i` parameter, or paste your json as a single line.

Pretty-printed JSONs through -stdin will fail. This may need some work.

### How to generate an input schema
This CLI was built with the intention of supporting the `- Tailwind` Export of [coolors.co](https://coolors.co/generate).

This is a good starting point for working with your palettes!

If there is a need for quickly switching between palettes (as opposed to the current constant colors generated), please let me know!

### Output

The above input will generate the following:

```rust
/// Generated using `color-gen` v0.1
use bevy::color::Srgba;
///Original hex: #462001
pub const SANDY_BROWN_100: Srgba = Srgba::rgb(
    0.27450982f32,
    0.1254902f32,
    0.003921569f32,
);
///Original hex: #fdbc87
pub const SANDY_BROWN_600: Srgba = Srgba::rgb(0.99215686f32, 0.7372549f32, 0.5294118f32);
///Original hex: #feeee1
pub const SANDY_BROWN_900: Srgba = Srgba::rgb(
    0.99607843f32,
    0.93333334f32,
    0.88235295f32,
);
///Original hex: #e3b1af
pub const REDWOOD_800: Srgba = Srgba::rgb(0.8901961f32, 0.69411767f32, 0.6862745f32);
///Original hex: #f1d8d7
pub const REDWOOD_900: Srgba = Srgba::rgb(0.94509804f32, 0.84705883f32, 0.84313726f32);
///Original hex: #d58a8792
pub const REDWOOD_700: Srgba = Srgba::rgba(
    0.8352941f32,
    0.5411765f32,
    0.5294118f32,
    0.57254905f32,
);
/**Error parsing hex #GGGGGG
r: Err(ParseIntError { kind: InvalidDigit })
g: Err(ParseIntError { kind: InvalidDigit })
b: Err(ParseIntError { kind: InvalidDigit })*/
pub const REDWOOD_NO_WORK: () = ();
```

If you specify an output `-o`, the output will save to that file. Otherwise, it will be printed to stdout.

## Help/Suggestions wanted!

Please let me know what formats should be supported. Additionally, I've got some thoughts:

- it could be nice to make strongly typed palettes with enumerations
- Other schema types
- correct parsing of JS Objects (most scheme generators return this, not JSON)
