use open_macros::*;

#[test]
fn string_macros_work() {
    assert_eq!(str_make!("abc"), String::from("abc"));
    assert_eq!(upper!("ak"), "AK");
    assert_eq!(lower!("AK"), "ak");
}

#[test]
fn number_guards_work() {
    assert_eq!(pos!(10).expect("positive value"), 10usize);
    assert!(pos!(-1).is_err());

    assert_eq!(neg!(-10).expect("negative value"), -10isize);
    assert!(neg!(1).is_err());
}

#[test]
fn control_flow_macros_work() {
    let mut value = 0;
    when!(true => {
        value = 3;
    });
    assert_eq!(value, 3);

    let mut with_else = Vec::new();
    when!(false => {
        with_else.push(1);
    }, else => {
        with_else.push(2);
    });
    assert_eq!(with_else, vec![2]);
}

#[test]
fn loop_macros_work() {
    let mut sum = 0;
    repeat!(i in 0 => 4, {
        sum += i;
    });
    assert_eq!(sum, 6);

    let items = [1, 2, 3];
    let mut seen = 0;
    each!(item in items, {
        seen += item;
    });
    assert_eq!(seen, 6);
}

#[test]
fn time_macros_return_reasonable_values() {
    let month = month_now!();
    assert!((1..=13).contains(&month));
    let year = year_now!();
    assert!(year >= 1970);
}

#[test]
fn retry_macro_retries_until_success() {
    let mut attempts = 0;
    let result: AkResult<&str> = retry!(3 => {
        attempts += 1;
        if attempts < 3 {
            Err(AkError::Validation("not yet"))
        } else {
            Ok("ready")
        }
    });

    assert_eq!(result.expect("eventual success"), "ready");
    assert_eq!(attempts, 3);
}

#[test]
fn retry_macro_can_delay_between_attempts() {
    let mut attempts = 0;
    let result: AkResult<&str> = retry!(2 => {
        attempts += 1;
        Err(AkError::Validation("always fail"))
    }, delay_ms => 1);

    assert!(result.is_err());
    assert_eq!(attempts, 2);
}

#[test]
fn measure_ms_macro_returns_value_and_elapsed_time() {
    let (result, elapsed_ms) = measure_ms!({
        let mut count = 0;
        repeat!(i in 0 => 3, {
            count += i;
        });
        count
    });

    assert_eq!(result, 3);
    assert!(elapsed_ms <= 5_000);
}

fn guarded_positive(input: i64) -> AkResult<usize> {
    ensure!(input > 0, "input must be greater than zero");
    pos!(input)
}

#[test]
fn ensure_macro_validates_and_exits_early() {
    assert_eq!(guarded_positive(4).expect("valid input"), 4usize);

    let err = guarded_positive(0).expect_err("invalid input should fail");
    match err {
        AkError::Validation(message) => assert_eq!(message, "input must be greater than zero"),
        _ => panic!("expected validation error"),
    }
}
