#[allow(unused_imports)]
pub(crate) use crate::testing::common::{Literal, Identifier};

#[allow(non_snake_case)]
macro_rules! Closure {
    ((fn($($arguments:tt)*) -> $($body:tt)+)) => (
        $crate::expression::Closure {
            arguments: $crate::punctuated::Punctuated::from_iter(
                $crate::segments!(
                    $crate::testing::__map[[$crate::testing::__grouped: ()] [[$crate::testing::Identifier]]]
                    where [,] in $($arguments)*
                ),
                $crate::punctuated::Punctuation::Comma,
            ),
            body: $crate::testing::construct_ast!([Expression] $($body)*),
        }
    );
    ((box $($tokens:tt)*)) => (
        Box::new($crate::testing::Closure!(($($tokens)*)))
    );
} pub(crate) use Closure;

#[allow(non_snake_case)]
macro_rules! Block {({ $($body:tt)* }) => (
    $crate::expression::Block {
        statements: $crate::punctuated::Punctuated::from_iter(
            $crate::segments!(
                $crate::testing::__map [[$crate::testing::construct_ast] [Statement]]
                where [;] in $($body)*
            ),
            $crate::punctuated::Punctuation::Semicolon,
        )
    }
)} pub(crate) use Block;

#[allow(non_snake_case)]
macro_rules! Expression {
    ($variant:ident $tt:tt) => (
        $crate::expression::Expression::$variant(
            $crate::testing::construct_ast!($variant $tt)
        )
    );
    ($inner:ident :: $variant:ident $tt:tt) => (
        $crate::expression::Expression::$inner(
            $crate::testing::construct_ast!($inner::$variant $tt)
        )
    );
    ($variant:ident) => ({
        ::core::compile_error!(
            ::core::concat!("missing grouped tts after Expression::", ::core::stringify!($variant))
        );
        $crate::expression::Expression::$variant;
    });
} pub(crate) use Expression;
