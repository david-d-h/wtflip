#![feature(generic_const_exprs)]
#![feature(macro_metavar_expr)]
#![feature(decl_macro)]
#![feature(unsized_const_params)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]

pub use defile::defile;

use std::hint::unreachable_unchecked;

pub const fn is_punct<const C: char>() -> bool {
    match C {
        '+' | '-' | '*' | '/' |
        '%' | '^' | '!' | '&' |
        '|' | '<' | '>' | '=' |
        ':' | '.' | ',' | ';' |
        '#' | '$' | '?' | '~' => true,
        _ => false,
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Punctuation {
    Plus, Minus, Star, Slash,
    Percent, Power, Bang, Amp,
    Pipe, Lesser, Greater, Eq,
    Colon, Dot, Comma, Semicolon,
    Pound, Dollar, Question, Tilde,
}

impl Punctuation {
    pub const fn from_char<const C: char>() -> Punctuation
    where
        [(); is_punct::<C>() as usize - 1]:
    {
        match C {
            '+' => Punctuation::Plus,
            '-' => Punctuation::Minus,
            '*' => Punctuation::Star,
            '/' => Punctuation::Slash,
            '%' => Punctuation::Percent,
            '^' => Punctuation::Power,
            '!' => Punctuation::Bang,
            '&' => Punctuation::Amp,
            '|' => Punctuation::Pipe,
            '<' => Punctuation::Lesser,
            '>' => Punctuation::Greater,
            '=' => Punctuation::Eq,
            ':' => Punctuation::Colon,
            '.' => Punctuation::Dot,
            ',' => Punctuation::Comma,
            ';' => Punctuation::Semicolon,
            '#' => Punctuation::Pound,
            '$' => Punctuation::Dollar,
            '?' => Punctuation::Dollar,
            '~' => Punctuation::Tilde,
            _ => unsafe { unreachable_unchecked() },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Punctuated<T> {
    pub punctuation: Punctuation,
    pub items: Vec<T>,
}

#[macro_export]
#[allow(non_snake_case)]
macro_rules! Punctuated {(match $char:tt use $item:path: $($tokens:tt)*) => ({
    const C: char = {
        #[inline]
        const fn char<const FROM: &'static str>() -> char
        where
            [(); { FROM.len() == 1 } as usize - 1]:
        {
            ::core::primitive::str::as_bytes(FROM)[0] as char
        }

        char::<{ ::core::stringify!($char) }>()
    };

    macro_rules! __punctuated_internal_parser {
        ([$$([$$($$output:tt)*])*] [$$($$buffer:tt)*] $char $$($$tail:tt)*) => (__punctuated_internal_parser!(
            [$$([$$($$output)*])* [$item!($$($$buffer)*)]] [] $$($$tail)*
        ));
        ([$$([$$($$output:tt)*]),*] [$$($$buffer:tt)*] $$tt:tt $$($$tail:tt)*) => (__punctuated_internal_parser!(
            [$$([$$($$output)*])*] [$$($$buffer)* $$tt] $$($$tail)*
        ));
        ([$$([$$($$output:tt)*])*] [$$($$tt:tt $$($$buffer:tt)*)?] $$(,)?) => ($crate::defile!({
            $crate::Punctuated::<_> {
                punctuation: $crate::Punctuation::from_char::<C>(),
                items: ::std::vec::Vec::from([$$($$($$output)*,)* $$(@$item!($$tt $$($$buffer)*))?])
            }
        }));
    }

    __punctuated_internal_parser!([] [] $($tokens)*)
})}

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

#[macro_export]
#[allow(non_snake_case)]
macro_rules! Literal {($literal:literal) => ({
    ::core::convert::Into::<$crate::Literal>::into($literal)
})}

def_literal! {
    Bool(bool),
    String(&'static str),
    I32(i32),
    F64(f64),
}

#[derive(Debug, Clone)]
pub struct Identifier(pub &'static str);

#[macro_export]
#[allow(non_snake_case)]
macro_rules! Identifier {($ident:ident) => {{
    $crate::Identifier(::core::stringify!($ident))
}}}

#[derive(Debug, Clone)]
pub struct VarDeclaration {
    pub name: Identifier,
    pub mutable: bool,
    pub value: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct VarAssignment {
    pub name: Identifier,
    pub value: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Value(Identifier),
    VarDeclaration(VarDeclaration),
    VarAssignment(VarAssignment),
}

#[macro_export]
#[allow(non_snake_case)]
macro_rules! Expr {
    ($name:ident $(mut $(@$($_:tt)* $is_mut:tt)?)? := $($expr:tt)*) => ({
        Expression::VarDeclaration(VarDeclaration {
            name: $crate::Identifier!($name),
            mutable: false | $($($is_mut)? true)?,
            value: ::std::boxed::Box::new($crate::Expr!($($expr)*)),
        })
    });
    ($name:ident = $($expr:tt)*) => ({
        Expression::VarAssignment(VarAssignment {
            name: $crate::Identifier!($name),
            value: ::std::boxed::Box::new($crate::Expr!($($expr)*)),
        })
    });
    ($ident:ident) => (const {
        Expression::Value($crate::Identifier!($ident))
    });
    ($literal:literal) => ({
        $crate::Expression::Literal($crate::Literal!($literal))
    });
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Identifier,
    pub args: Punctuated<Identifier>,
    pub body: Punctuated<Statement>,
}

#[derive(Debug, Clone)]
pub struct ShortFunction {
    pub name: Identifier,
    pub args: Punctuated<Identifier>,
    pub body: Expression,
}

#[derive(Debug, Clone)]
pub enum Statement {
    FnDeclaration(Function),
    ShortFnDeclaration(ShortFunction),
    Expr(Expression),
}

#[macro_export]
#[allow(non_snake_case)]
macro_rules! Terminated {(match [$($delim:tt)*] use $item:path: $($tokens:tt)*) => ({
    macro_rules! __terminated_internal_parser {
        ([$$($$output:tt)*] $($delim)*) => ($crate::defile!({
            @$item!($$($$output)*)
        }));
        ([$$($$output:tt)*] $$tt:tt $$($$tail:tt)*) => (__terminated_internal_parser!(
            [$$($$output)* $$tt] $$($$tail)*
        ));
    }

    __terminated_internal_parser!([] $($tokens)*)
})}

#[macro_export]
#[allow(non_snake_case)]
macro_rules! Statement {
    (fn $name:ident ($($arguments:tt)*) = $($body:tt)*) => ({
        $crate::Statement::ShortFnDeclaration($crate::ShortFunction {
            name: $crate::Identifier!($name),
            args: $crate::Punctuated!(match , use $crate::Identifier: $($arguments)*),
            body: $crate::Terminated!(match [;] use $crate::Expr: $($body)*),
        })
    });
    (fn $name:ident ($($arguments:tt)*) { $($body:tt)* }) => ({
        $crate::Statement::FnDeclaration($crate::Function {
            name: $crate::Identifier!($name),
            args: $crate::Punctuated!(match , use $crate::Expr: $($arguments)*),
            body: $crate::Punctuated!(match ; use $crate::Statement: $($body)*),
        })
    });
    ($($tokens:tt)*) => ({
        $crate::Statement::Expr($crate::Expr!($($tokens)*))
    });
}
