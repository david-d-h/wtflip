pub(crate) use crate::testing::expression::Expression;

#[allow(non_snake_case)]
macro_rules! VarDeclaration {({ $($tokens:tt)* }) => (
    $crate::segments!(
        $crate::testing::__construct_from_fields[$crate::statement::VarDeclaration]
        where [,] in $($tokens)*
    )
)} pub(crate) use VarDeclaration;

#[allow(non_snake_case)]
macro_rules! VarAssignment {({ $($tokens:tt)* }) => (
    $crate::segments!(
        $crate::testing::__construct_from_fields[$crate::statement::VarAssignment]
        where [,] in $($tokens)*
    )
)} pub(crate) use VarAssignment;

#[allow(non_snake_case)]
macro_rules! Return {(($($tokens:tt)*)) => (
    $crate::__optional!([$crate::testing::construct_ast[Expression]] $($tokens)*)
)} pub(crate) use Return;

#[allow(non_snake_case)]
macro_rules! Statement {
    ($variant:ident $tt:tt) => (
        $crate::statement::Statement::$variant(
            $crate::testing::statement::$variant!($tt),
        )
    );
    ($inner:ident :: $variant:ident $tt:tt) => (
        $crate::statement::Statement::$inner(
            $crate::testing::construct_ast!($inner::$variant $tt)
        )
    );
    ($variant:ident) => ({
        ::core::compile_error!(
            ::core::concat!("missing grouped tts after Statement::", ::core::stringify!($variant))
        );
        $crate::statement::Statement::$variant;
    });
} pub(crate) use Statement;