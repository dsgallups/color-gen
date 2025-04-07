use cargo_color_gen::generate;
use serde_json::json;

const JSON: &str = r##"{"sandy_brown":{"100":"#462001","600":"#fdbc87","900":"#feeee1"},"redwood":{"700":"#d58a8792","800":"#e3b1af","900":"#f1d8d7","NO_WORK":"#GGGGGG", "NO_WORK_TWO":"#d"}}"##;

#[test]
fn parse_json() {
    generate(JSON.to_string()).unwrap();
}

const OBJ: &str = r##"{ 'sage': { DEFAULT: '#a3a380', 100: '#222218' }}"##;

#[test]
fn parse_obj() {
    generate(OBJ.to_string()).unwrap();
}

#[test]
fn sandbox() {
    // let config = "
    //     {
    //       // A traditional message.
    //       message: 'hello world',

    //       // A number for some reason.
    //       n: 42,
    //     }
    // ";
    let config = "
        {
            'sage': {
                DEFAULT: '#a3a380',
                100: '#222218'
            }
        }
    ";

    assert_eq!(
        json5::from_str::<serde_json::Value>(&config),
        Ok(json!({
            "message": "hello world",
            "n": 42
        }))
    );
    //let json: serde_json::Value = json5::from_str(OBJ).unwrap();
}
