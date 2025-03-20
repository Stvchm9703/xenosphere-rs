#[cfg(test)]
#[test]
fn test_parser_with_single_file() {
    // create test cli cmd
    let mut cmd = Command::new("xenosphere-parser");
    cmd.arg("parse");
    cmd.arg("../../concept/packages/concept-1.xesl");
    cmd.assert().success();
    println!("{:?}", cmd.output());
}
