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
