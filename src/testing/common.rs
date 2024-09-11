#[allow(non_snake_case)]
macro_rules! Literal {(($literal:literal)) => ({
    ::core::convert::Into::<$crate::common::Literal>::into($literal)
})} pub(crate) use Literal;

#[allow(non_snake_case)]
macro_rules! Identifier {(($identifier:ident)) => ({
    $crate::common::Identifier(::core::stringify!($identifier))
})} pub(crate) use Identifier;
