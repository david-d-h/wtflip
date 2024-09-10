use crate::common;

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(common::Literal),
    Value(common::Identifier),
}

#[macro_export]
#[allow(non_snake_case)]
macro_rules! Expr {
    ($ident:ident) => (const {
        $crate::expression::Expression::Value($crate::Identifier!($ident))
    });
    ($literal:literal) => ({
        $crate::expression::Expression::Literal($crate::Literal!($literal))
    });
}