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

macro_rules! punctuation_from_char {
    (+) => ($crate::punctuated::Punctuation::Plus);
    (-) => ($crate::punctuated::Punctuation::Minus);
    (*) => ($crate::punctuated::Punctuation::Star);
    (/) => ($crate::punctuated::Punctuation::Slash);
    (%) => ($crate::punctuated::Punctuation::Percent);
    (^) => ($crate::punctuated::Punctuation::Power);
    (!) => ($crate::punctuated::Punctuation::Bang);
    (&) => ($crate::punctuated::Punctuation::Amp);
    (|) => ($crate::punctuated::Punctuation::Pipe);
    (<) => ($crate::punctuated::Punctuation::Lesser);
    (>) => ($crate::punctuated::Punctuation::Greater);
    (=) => ($crate::punctuated::Punctuation::Eq);
    (:) => ($crate::punctuated::Punctuation::Colon);
    (.) => ($crate::punctuated::Punctuation::Dot);
    (,) => ($crate::punctuated::Punctuation::Comma);
    (;) => ($crate::punctuated::Punctuation::Semicolon);
    (#) => ($crate::punctuated::Punctuation::Pound);
    // ($$) => ($crate::punctuated::Punctuation::Dollar); can't match on dollar sign
    (?) => ($crate::punctuated::Punctuation::Question);
    (~) => ($crate::punctuated::Punctuation::Tilde);
} pub(crate) use punctuation_from_char;

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

#[derive(Debug, Clone, PartialEq)]
pub struct Punctuated<T> {
    pub punctuation: Punctuation,
    pub items: Vec<T>,
}

impl<T> Punctuated<T> {
    pub fn from_iter<I>(iter: I, punctuation: Punctuation) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            punctuation,
            items: iter.into_iter().collect(),
        }
    }
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
        ([$$([$$($$output:tt)*])*] [$$($$buffer:tt)*] $$tt:tt $$($$tail:tt)*) => (__punctuated_internal_parser!(
            [$$([$$($$output)*])*] [$$($$buffer)* $$tt] $$($$tail)*
        ));
        ([$$([$$($$output:tt)*])*] [$$($$tt:tt $$($$buffer:tt)*)?] $$($char)?) => ($crate::defile!({
            $crate::punctuated::Punctuated::<_> {
                punctuation: $crate::punctuated::Punctuation::from_char::<C>(),
                items: ::std::vec::Vec::from([$$($$($$output)*,)* $$(@$item!($$tt $$($$buffer)*))?])
            }
        }));
    }

    __punctuated_internal_parser!([] [] $($tokens)*)
})}