use core::cmp;

/// This macro returns the name of the enclosing function.
///
/// It is ported from stdext::function_name to work without std.
/// Ref: https://docs.rs/stdext/0.2.1/stdext/macro.function_name.html
#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            core::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3] // `3` is the length of the `::f`.
    }};
}

/// Calculate the digit width of a number, like 100 = 3.
///
/// This function is necessary since we can't use f32.log32() that is part of std, and we can't
/// easily use intrinsics either.
#[allow(dead_code)]
pub fn digit_width(n: usize) -> usize {
    let mut digits = 0;
    let mut m = n;
    while m > 0 {
        m /= 10;
        digits += 1
    }
    cmp::max(1, digits)
}

#[test_case]
fn digit_width() {
    assert_eq!(1, digit_width(0));
    assert_eq!(1, digit_width(1));
    assert_eq!(2, digit_width(10));
    assert_eq!(3, digit_width(100));
    assert_eq!(4, digit_width(1000));
}
