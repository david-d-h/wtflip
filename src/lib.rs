#![feature(generic_const_exprs)]
#![feature(macro_metavar_expr)]
#![feature(decl_macro)]
#![feature(unsized_const_params)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]

pub mod common;
pub mod punctuated;
pub mod terminated;
pub mod expression;
pub mod statement;
pub mod item;
pub mod util;

pub use defile::defile;

pub(in crate) mod testing {
    macro_rules! test_ast {($ast:ident -> $($tokens:tt)*) => ({
        let check = $crate::testing::construct_ast!($($tokens)*);
        assert_eq!(&check, &$ast);
    })} pub(in crate) use test_ast;

    #[allow(non_snake_case)]
    macro_rules! Expression {($variant:ident $tt:tt) => ({
        $crate::expression::Expression::$variant(
            $crate::testing::$variant!($tt)
        )
    })} pub(in crate) use Expression;

    #[allow(non_snake_case)]
    macro_rules! Literal {(($literal:literal)) => ({
        ::core::convert::Into::<$crate::common::Literal>::into($literal)
    })} pub(in crate) use Literal;
    
    macro_rules! construct_ast {
        ($inner:ident $tt:tt) => ({
            $crate::testing::$inner!($tt)
        });
        ($inner:ident :: $variant:ident $tt:tt) => ({
            $crate::testing::$inner!($variant $tt)
        });
    } pub(in crate) use construct_ast;

    #[test]
    #[cfg(test)]
    fn it_works() {
        let ast = crate::Expression!("hallo :3");

        test_ast! { ast ->
            Expression::Literal("hallo :3")
        }
    }
}
