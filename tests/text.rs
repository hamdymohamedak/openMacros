use open_macros::*;

#[test]
fn contains_macro_works() {
    assert!(contains!("hello world", "world"));
    assert!(!contains!("hello", "world"));
}

#[test]
fn replace_macros_work() {
    assert_eq!(replace!("a-b-c", "-", "/"), "a/b-c");
    assert_eq!(replace_all!("a-b-c", "-", "/"), "a/b/c");
}

fn tpl_run() -> open_macros::AkResult<()> {
    let msg = tpl!("Hello {{who}}!", who = "Rust")?;
    assert_eq!(msg, "Hello Rust!");
    assert!(contains!(msg, "Rust"));
    Ok(())
}

#[test]
fn tpl_macro_works() {
    tpl_run().expect("template should render");
}

#[test]
fn tpl_errors_on_unresolved_placeholder() {
    let err = tpl!("Hello {{missing}}!").expect_err("unresolved placeholder");
    match err {
        open_macros::AkError::Validation(msg) => {
            assert_eq!(msg, "template contains unresolved placeholders");
        }
        other => panic!("expected validation error, got {other:?}"),
    }
}

#[cfg(feature = "text")]
#[test]
fn regex_macros_work() {
    fn run() -> open_macros::AkResult<()> {
        assert!(regex_match!("abc123", r"\d+")?);
        assert_eq!(regex_find!("abc123", r"\d+")?, Some(String::from("123")));
        assert_eq!(regex_replace!("a  b", r"\s+", " ")?, "a b");
        assert_eq!(regex_split!("a,b,c", ",")?, vec!["a", "b", "c"]);
        Ok(())
    }
    run().expect("regex macros should work");
}
