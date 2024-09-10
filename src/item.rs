use crate::{common, expression, punctuated, statement};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: common::Identifier,
    pub args: punctuated::Punctuated<common::Identifier>,
    pub body: punctuated::Punctuated<statement::Statement>,
}

#[derive(Debug, Clone)]
pub struct ShortFunction {
    pub name: common::Identifier,
    pub args: punctuated::Punctuated<common::Identifier>,
    pub body: expression::Expression,
}

#[derive(Debug, Clone)]
pub enum Item {
    Fn(Function),
    ShortFn(ShortFunction),
}

#[macro_export]
#[allow(non_snake_case)]
macro_rules! Item {
    (fn $name:ident ($($arguments:tt)*) { $($body:tt)* }) => ({
        $crate::item::Item::Fn($crate::item::Function {
            name: $crate::Identifier!($name),
            args: $crate::Punctuated!(match , use $crate::Expr: $($arguments)*),
            body: $crate::Punctuated!(match ; use $crate::Statement: $($body)*),
        })
    });
    (fn $name:ident ($($arguments:tt)*) = $($body:tt)*) => ({
        $crate::item::Item::ShortFn($crate::item::ShortFunction {
            name: $crate::Identifier!($name),
            args: $crate::Punctuated!(match , use $crate::Identifier: $($arguments)*),
            body: $crate::Terminated!(match [;] use $crate::Expr: $($body)*),
        })
    });
}