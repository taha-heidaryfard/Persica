use std::{
    io::Write,
    process::{Command, Stdio},
};

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_persica"))
}

#[test]
fn eval_executes_inline_source() {
    let output = binary().args(["--eval", "print(6 * 7);"]).output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap().trim(), "42");
}

#[test]
fn check_validates_without_running() {
    let output = binary()
        .args(["--check", "--eval", "print(1 / 0);"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains("check succeeded")
    );
}

#[test]
fn diagnostics_are_printed_to_stderr() {
    let output = binary()
        .args(["--eval", "int value = true;"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("semantic error"));
    assert!(stderr.contains("expected int, found bool"));
    assert!(stderr.contains("<eval>:1:13"));
}

#[test]
fn dash_reads_source_from_stdin() {
    let mut child = binary()
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"print(21 * 2);")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap().trim(), "42");
}

#[test]
fn read_line_uses_process_stdin() {
    let mut child = binary()
        .args([
            "--eval",
            "string name = read_line(); print(\"Hello \" + name);",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.as_mut().unwrap().write_all(b"Taha\n").unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap().trim(),
        "Hello Taha"
    );
}
