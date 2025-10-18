use std::path::PathBuf;

use oxjot::file_util::read_file;
use oxjot::parser::parse_document;
use oxjot::render::convert_ast_to_html;

#[test]
fn compare_fixtures() {
    run_case("input.djot", "expected.html");
}

fn run_case(djot_name: &str, html_name: &str) {
    let djot_text = read_fixtures(&format!("{djot_name}"));
    let expected = read_fixtures(&format!("{html_name}"));

    let doc = parse_document(&djot_text);
    let got = convert_ast_to_html(&doc, djot_text.as_bytes());

    assert_eq!(expected, got, "Mismatch for {djot_name} -> {html_name}");
}

fn read_fixtures(rel: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(rel);
    read_file(&path).unwrap_or_else(|e| panic!("read {:?}: {}", path, e))
}
