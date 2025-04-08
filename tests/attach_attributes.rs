use fixture::{add_1_to_function, add_1_to_implementation};
use getset::{CopyGetters, Getters};

// #[allow(unused)]
// #[add_1_to_function]
// fn two() -> u8 {
//     2
// }

// struct Test;

// #[cfg(target_os = "linux")]
// #[add_1_to_implementation]
// impl Test {
//     #[add_1_to_function]
//     #[allow(unused)]
//     fn two(&self) -> u8 {
//         2
//     }
//     #[allow(unused)]
//     #[add_1_to_function]
//     fn three(&self) -> u8 {
//         3
//     }
// }

// #[test]
// fn simple() {
//     assert_eq!(two(), 3, "Function attribute not applied correctly.");
//     assert_eq!(
//         Test {}.two(),
//         4,
//         "Implementation attribute not applied correctly."
//     );
//     assert_eq!(
//         Test {}.three(),
//         5,
//         "Implementation attribute not applied correctly."
//     );
// }

// try with setters too to make sure doesn't blow up
// try passing it in individual fields and see what it does, it shouldn't allow it
// try changing order to see if it matters
// #[getset(get_copy, attrs=[add_1_to_implementation, allow(unused), cfg(target_os = "linux")])]
// #[getset(get_copy)]
// #[getset(get_copy, attrs=[add_1_to_implementation])]
// #[getset(get_copy, attrs(hi, there, man))]
// #[getset(get_copy, attrs = r#"#[add_1_to_implementation]"#)]
#[derive(CopyGetters)]
#[getset(
    // get = "with_prefix",
    get_copy,
    impl_attrs = r#"
    #[cfg(target_os = "linux")]
    #[add_1_to_implementation]
    #[allow(unused)]
"#
)]
struct Test {
    a: u8,
    b: u8,
}

#[test]
fn simple() {
    let test_instance = Test { a: 1, b: 2 };
    assert_eq!(
        test_instance.a(),
        2,
        "Function attribute not applied correctly."
    );
    assert_eq!(
        test_instance.b(),
        3,
        "Function attribute not applied correctly."
    );
}
