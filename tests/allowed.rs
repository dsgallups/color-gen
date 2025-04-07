use cargo_color_gen::generate;

const JSON: &str = r##"{"sandy_brown":{"100":"#462001","600":"#fdbc87","900":"#feeee1"},"redwood":{"700":"#d58a8792","800":"#e3b1af","900":"#f1d8d7","NO_WORK":"#GGGGGG", "NO_WORK_TWO":"#d"}}"##;

#[test]
fn parse_json() {
    generate(JSON.to_string()).unwrap();
}
