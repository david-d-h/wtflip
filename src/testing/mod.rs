macro_rules! export_local {($($ident:ident::*),* $(,)?) => {$(
    pub(crate) mod $ident;
    pub(crate) use $ident::*;
)*}} use export_local;

export_local![
    common::*,
    expression::*,
    item::*,
    statement::*,
    util::*,
];

macro_rules! test_ast {($ast:ident -> $($tokens:tt)*) => ({
    assert_eq!(
        &$crate::testing::construct_ast!($($tokens)*),
        &$ast,
    );
})} pub(crate) use test_ast;

macro_rules! construct_ast {
    ($inner:ident $tt:tt) => ({
        $crate::testing::$inner!($tt)
    });
    ($inner:ident :: $variant:ident $tt:tt ) => ({
        $crate::testing::$inner!($variant $tt)
    });
    // tokens may be received in a callback format
    ([$($tokens:tt)*] $($parse:tt)*) => ($crate::testing::construct_ast!(
        @passthrough $($tokens)* ( $($parse)* )
    ));
    (@passthrough $inner:ident ( $($tokens:tt)* )) => (
        $crate::testing::$inner!($($tokens)*)
    );
    ($inner:ident) => ({
        $crate::testing::$inner;
    });
    // for IDE analysis purposes we allow not having a tt following the
    // inner parser, handling this is up to individual parser implementation.
    ($inner:ident :: $variant:ident) => ({
        $crate::testing::$inner!($variant)
    });
} pub(crate) use construct_ast;
