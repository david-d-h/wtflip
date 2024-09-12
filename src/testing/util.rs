macro_rules! __construct_from_fields {([$item:path] $([$field:ident: $($tokens:tt)*])*) => ({
    macro_rules! __fields_internal_parser {
        ([$$($$output:tt)*] [$$field:ident: #$$($$inner:tt)*] $$($$tail:tt)*) => (__fields_internal_parser!(
            [$$($$output)* $$field: $crate::testing::construct_ast!($$($$inner)*),] $$($$tail)*
        ));
        ([$$($$output:tt)*] [$$field:ident: $$expr:expr] $$($$tail:tt)*) => (__fields_internal_parser!(
            [$$($$output)* $$field: $$expr,] $$($$tail)*
        ));
        ([$$($$output:tt)*]) => ({
            $item { $$($$output)* }
        })
    }

    __fields_internal_parser!([] $([$field: $($tokens)*])*)
})} pub(crate) use __construct_from_fields;

macro_rules! __punctuated {(match $char:tt use $item:path: $($tokens:tt)*) => ({
    let punctuation = $crate::punctuated::punctuation_from_char!($char);
    $crate::punctuated::Punctuated::from_iter($crate::segments!(
        $crate::__map[$item]
        where [$char] in $($tokens)*
    ), punctuation)
})} pub(crate) use __punctuated;

macro_rules! __grouped {([[$callback:path] $([$($carry:tt)*])?] $kind:tt $($tokens:tt)*) => ($crate::defile!({
    macro_rules! __grouper {
        (()) => (@$callback!($([$($carry)*])? ( $($tokens)* )));
        ({}) => (@$callback!($([$($carry)*])? { $($tokens)* }));
        ([]) => (@$callback!($([$($carry)*])? [ $($tokens)* ]));
    }

    __grouper!($kind)
}))} pub(crate) use __grouped;
