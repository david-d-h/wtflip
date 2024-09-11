macro_rules! __construct_from_fields {([$item:path] $([$field:ident: $($tokens:tt)*])*) => ({
    macro_rules! __fields_internal_parser {
        ([$$($$output:tt)*] [$$field:ident: @$$($$inner:tt)*] $$($$tail:tt)*) => (__fields_internal_parser!(
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
