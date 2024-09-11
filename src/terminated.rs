#[macro_export]
#[allow(non_snake_case)]
macro_rules! Terminated {(match [$($delim:tt)*] use $item:path: $($tokens:tt)*) => ({
    macro_rules! __terminated_internal_parser {
        ([$$($$output:tt)*] $($delim)*) => ({
            $crate::defile(@$item)!($$($$output)*)
        });
        ([$$($$output:tt)*] $$tt:tt $$($$tail:tt)*) => (__terminated_internal_parser!(
            [$$($$output)* $$tt] $$($$tail)*
        ));
    } use __terminated_internal_parser;

    __terminated_internal_parser!([] $($tokens)*)
})}