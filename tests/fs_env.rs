use open_macros::*;
use std::fs;

fn file_read_write_and_exists_run() -> open_macros::AkResult<()> {
    let path = "target/om-test-file.txt";
    let _ = fs::remove_file(path);
    file_write!(path, "hello v2")?;
    assert!(file_exists!(path));
    let content = file_read!(path)?;
    assert_eq!(content, "hello v2");
    let _ = file_rm!(path);
    Ok(())
}

#[test]
fn file_read_write_and_exists_work() {
    file_read_write_and_exists_run().expect("fs macros should work");
}

fn dir_mk_and_lines_run() -> open_macros::AkResult<()> {
    let dir = path_join!("target", "om-test-dir");
    let _ = dir_wipe!(&dir);
    dir_mkp!(&dir)?;
    let file = dir.join("lines.txt");
    file_write!(&file, "a\nb\nc")?;
    let all_lines = lines!(&file)?;
    assert_eq!(all_lines, vec!["a", "b", "c"]);
    let _ = dir_wipe!(&dir);
    Ok(())
}

#[test]
fn dir_mk_and_lines_work() {
    dir_mk_and_lines_run().expect("dir and lines macros should work");
}

fn env_macros_run() -> open_macros::AkResult<()> {
    env_set!("OM_TEST_VAR", "ok")?;
    let value = env_get!("OM_TEST_VAR")?;
    assert_eq!(value, "ok");
    assert_eq!(env_or!("OM_MISSING_VAR", "fallback"), "fallback");
    Ok(())
}

#[test]
fn env_macros_work() {
    env_macros_run().expect("env macros should work");
}

fn parse_and_string_helpers_run() -> open_macros::AkResult<()> {
    assert_eq!(parse_int!("42")?, 42);
    assert_eq!(parse_float!("3.5")?, 3.5);
    assert_eq!(trim!("  hi  "), "hi");
    assert_eq!(split!("a,b", ","), vec!["a", "b"]);
    Ok(())
}

#[test]
fn parse_and_string_helpers_work() {
    parse_and_string_helpers_run().expect("parse and string helpers should work");
}
