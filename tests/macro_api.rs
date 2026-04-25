use ak_macros::*;

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
