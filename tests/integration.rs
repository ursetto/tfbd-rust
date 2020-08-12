use assert_cmd::Command;

#[test]
fn midi_magic() {
    let contents = std::fs::read_to_string("tests/midi.magic.t.txt")
        .expect("comparison file missing");
    Command::cargo_bin("tfbd")
        .unwrap()
        .arg("decode")
        .arg("tests/midi.magic.t")
        .assert()
        .stdout(contents)
        .success();
}
