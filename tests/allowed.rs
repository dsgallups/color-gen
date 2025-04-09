use cargo_color_gen::generate;

const JSON: &str = r##"{"sandy_brown":{"100":"#462001","600":"#fdbc87","900":"#feeee1"},"redwood":{"700":"#d58a8792","800":"#e3b1af","900":"#f1d8d7","NO_WORK":"#GGGGGG", "NO_WORK_TWO":"#d"}}"##;

#[test]
fn parse_json() {
    generate(JSON).unwrap();
}

const OBJ: &str = r##"{ 'sage': { DEFAULT: '#a3a380', 100: '#222218' }}"##;

#[test]
fn parse_obj() {
    generate(OBJ).unwrap();
}

#[test]
fn parse_pretty_printed_object() {
    let config = "
        {
            'sage': {
                DEFAULT: '#a3a380',
                100: '#222218'
            }
        }
    ";
    generate(config).unwrap();
}
