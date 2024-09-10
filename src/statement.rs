use crate::{common, expression};

#[derive(Debug, Clone, PartialEq)]
pub struct VarDeclaration {
    pub name: common::Identifier,
    pub mutable: bool,
    pub value: expression::Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarAssignment {
    pub name: common::Identifier,
    pub value: expression::Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VarDeclaration(VarDeclaration),
    VarAssignment(VarAssignment),
    Expr(expression::Expression),
}

#[macro_export]
#[allow(non_snake_case)]
macro_rules! Statement {
    ($name:ident $(mut $(@$($_:tt)* $is_mut:tt)?)? := $($expr:tt)*) => ({
        $crate::statement::Statement::VarDeclaration(
            $crate::statement::VarDeclaration {
                name: $crate::Identifier!($name),
                mutable: false | $($($is_mut)? true)?,
                value: $crate::Expression!($($expr)*),
            },
        )
    });
    ($name:ident = $($expr:tt)*) => ({
        $crate::statement::Statement::VarAssignment(
            $crate::statement::VarAssignment {
                name: $crate::Identifier!($name),
                value: $crate::Expression!($($expr)*),
            },
        )
    });
    ($($tokens:tt)*) => ({
        $crate::statement::Statement::Expr($crate::Expression!($($tokens)*))
    });
}
