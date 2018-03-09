//! Common testing logic.


/// Create a new `BTreeMap` from the specified `key => value` pairs.
macro_rules! map {
    () => {
        ::std::collections::BTreeMap::new()
    };
    ($($key:expr => $val:expr),+$(,)*) => {
        {
            let mut map = ::std::collections::BTreeMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}


/// Generate tests for the specified values.
///
/// For serialization functions with a signature `fn (tag, value) -> Result`.
macro_rules! generate_tests_ser_1 {
    (
        $(use $serialize:path : $modname:ident),+$(,)*
        with
        $args:tt
    ) => {
        $(
            generate_tests_ser_1!(@mod $serialize, $modname, $args);
        )*
    };
    (@mod
        $serialize:path,
        $modname:ident,
        {$({
            case: $name:ident,
            tag: $tag:expr,
            value: $value:expr,
            expect: $expect:expr
            $(,)*
        }),*$(,)*}
    ) => {
        mod $modname {
            use super::*;

            $(
                #[test]
                fn $name() {
                    let value = $serialize(&$tag, &$value).unwrap();
                    assert_eq!(value, $expect);
                }
            )*
        }
    }
}

/// Generate tests for the specified values.
///
/// For serialization functions with a signature
/// `fn (tag_key, tag_value, value) -> Result`.
#[allow(unused)] // will be used later
macro_rules! generate_tests_ser_2 {
    (
        $(use $serialize:path : $modname:ident),+$(,)*
        with
        $args:tt
    ) => {
        $(
            generate_tests_ser_1!(@mod $serialize, $modname, $args);
        )*
    };
    (@mod
        $serialize:path,
        $modname:ident,
        {$({
            case: $name:ident,
            tag_k: $tag_key:expr,
            tag_v: $tag_val:expr,
            key: $val_key:expr,
            value: $value:expr,
            expect: $expect:expr
            $(,)*
        }),*$(,)*}
    ) => {
        mod $modname {
            use super::*;

            $(
                #[test]
                fn $name() {
                    let value = $serialize(&$tag_key, &$tag_val, &$value).unwrap();
                    assert_eq!(value, $expect);
                }
            )*
        }
    }
}

/// Generate tests for the specified values.
///
/// For serialization functions with a signature
/// `fn (tag_key, tag_value, value_key, value) -> Result`.
macro_rules! generate_tests_ser_3 {
    (
        $(use $serialize:path : $modname:ident),+$(,)*
        with
        $args:tt
    ) => {
        $(
            generate_tests_ser_3!(@mod $serialize, $modname, $args);
        )*
    };
    (@mod
        $serialize:path,
        $modname:ident,
        {$({
            case: $name:ident,
            tag_k: $tag_key:expr,
            tag_v: $tag_val:expr,
            key: $val_key:expr,
            value: $value:expr,
            expect: $expect:expr
            $(,)*
        }),*$(,)*}
    ) => {
        mod $modname {
            use super::*;

            $(
                #[test]
                fn $name() {
                    let value = $serialize(&$tag_key, &$tag_val, &$val_key, &$value).unwrap();
                    assert_eq!(value, $expect);
                }
            )*
        }
    }
}
