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
            '?' => Punctuation::Question,
            '~' => Punctuation::Tilde,
            _ => unsafe { unreachable_unchecked() },
        }
    }

    pub const fn to_char(&self) -> char {
        match self {
            Punctuation::Plus => '+',
            Punctuation::Minus => '-',
            Punctuation::Star => '*',
            Punctuation::Slash => '/',
            Punctuation::Percent => '%',
            Punctuation::Power => '^',
            Punctuation::Bang => '!',
            Punctuation::Amp => '&',
            Punctuation::Pipe => '|',
            Punctuation::Lesser => '<',
            Punctuation::Greater => '>',
            Punctuation::Eq => '=',
            Punctuation::Colon => ':',
            Punctuation::Dot => '.',
            Punctuation::Comma => ',',
            Punctuation::Semicolon => ';',
            Punctuation::Pound => '#',
            Punctuation::Dollar => '$',
            Punctuation::Question => '?',
            Punctuation::Tilde => '~',
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
            $crate::punctuated::Punctuated::<_> {
                punctuation: $crate::punctuated::Punctuation::from_char::<C>(),
                items: ::std::vec::Vec::from([$$($$($$output)*,)* $$(@$item!($$tt $$($$buffer)*))?])
            }
        }));
    }

    __punctuated_internal_parser!([] [] $($tokens)*)
})}