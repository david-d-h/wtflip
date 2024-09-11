use crate::{common, punctuated, statement};

#[derive(Debug, Clone, PartialEq)]
pub struct Closure {
    pub arguments: punctuated::Punctuated<common::Identifier>,
    pub body: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: punctuated::Punctuated<statement::Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Closure(Box<Closure>),
    Literal(common::Literal),
    Identifier(common::Identifier),
    Block(Block),
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
        $crate::expression::Expression::Identifier($crate::Identifier!($ident))
    });
    ($literal:literal) => ({
        $crate::expression::Expression::Literal($crate::Literal!($literal))
    });
    ({ $($block:tt)* }) => ({
        $crate::expression::Expression::Block($crate::Block!($($block)*))
    });
}

#[macro_export]
#[allow(non_snake_case)]
macro_rules! Block {($($body:tt)*) => ({
    $crate::expression::Block {
        statements: $crate::Punctuated!(match ; use $crate::Statement: $($body)*),
    }
})}

#[cfg(test)]
mod tests {
    use crate::testing;

    #[test]
    fn literal() {
        let ast = crate::Expression!(12.0);

        testing::test_ast! { ast ->
            Expression::Literal(12.0)
        }
    }

    #[test]
    fn ident_value() {
        let ast = crate::Expression!(hallo);

        testing::test_ast! { ast ->
            Expression::Identifier(hallo)
        }
    }

    #[test]
    fn empty_block() {
        let ast = crate::Expression!({});

        testing::test_ast! { ast ->
            Expression::Block {}
        }
    }

    #[test]
    fn block_multi_statement() {
        let ast = crate::Expression!({
            "hallo :3";
            jallo := 3;
        });

        testing::test_ast! { ast ->
            Expression::Block {
                Expression::Literal("hallo :3");
                VarDeclaration {
                    name: #Identifier(jallo),
                    mutable: false,
                    value: #Expression::Literal(3)
                };
            }
        }
    }
}
