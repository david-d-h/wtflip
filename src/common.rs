use crate::{punctuated, statement};

// IDENTIFIERS
#[derive(Debug, Clone)]
pub struct Identifier(pub &'static str);

#[macro_export]
#[allow(non_snake_case)]
macro_rules! Identifier {($ident:ident) => {{
    $crate::common::Identifier(::core::stringify!($ident))
}}}

// LITERALS
macro_rules! def_literal {($($variant:ident($ty:ty)),* $(,)?) => {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Literal {
        $($variant($ty),)*
    }

    $(impl ::core::convert::Into<Literal> for $ty {
        fn into(self) -> Literal {
            Literal::$variant(self)
        }
    })*
}}

def_literal! {
    Bool(bool),
    String(&'static str),
    I32(i32),
    F64(f64),
}

#[macro_export]
#[allow(non_snake_case)]
macro_rules! Literal {($literal:literal) => ({
    ::core::convert::Into::<$crate::common::Literal>::into($literal)
})}

// BLOCKS
#[derive(Debug, Clone)]
pub struct Block {
    pub statements: punctuated::Punctuated<statement::Statement>,
}

#[macro_export]
#[allow(non_snake_case)]
macro_rules! Block {($($body:tt)*) => ({
    $crate::common::Block {
        statements: $crate::Punctuated!(match ; use $crate::Statement: $($body)*),
    }
})}
