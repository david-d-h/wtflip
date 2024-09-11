use crate::{punctuated, common};

#[derive(Debug, Clone, PartialEq)]
pub struct Closure {
    pub arguments: punctuated::Punctuated<common::Identifier>,
    pub body: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Closure(Box<Closure>),
    Literal(common::Literal),
    Value(common::Identifier),
    Block(common::Block),
}

#[macro_export]
#[allow(non_snake_case)]
macro_rules! Expression {
    (fn($($arguments:tt)*) -> $($body:tt)*) => ({
        $crate::expression::Expression::Closure(Box::new($crate::expression::Closure {
            arguments: $crate::Punctuated!(match , use $crate::Identifier: $($arguments)*),
            body: $crate::Expression!($($body)*),
        }))
    });
    ($ident:ident) => (const {
        $crate::expression::Expression::Value($crate::Identifier!($ident))
    });
    ($literal:literal) => ({
        $crate::expression::Expression::Literal($crate::Literal!($literal))
    });
    ({ $($block:tt)* }) => ({
        $crate::expression::Expression::Block($crate::Block!($($block)*))
    });
}
