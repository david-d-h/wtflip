#[macro_export]
macro_rules! wtflip {
    /* Mutable declaration. */
    (@as_statement mut $ident:ident := $($expr:tt)+) => {
        let mut $ident = wtflip!(@as_expr $($expr)+);
    };

    /* Assignment or immutable declaration. */
    (@as_statement $ident:ident $(: $(@$($_:tt)* $declaration:tt)?)?= $($expr:tt)+) => {
        $($($declaration)? let)? $ident = wtflip!(@as_expr $($expr)+);
    };

    /* A compatibility layer. Allows for writing Rust in the wtflip invocation. */
    (@as_statement @{ $($compat:tt)* }) => ($($compat)*);

    /* Macro invocation. The macro invocation can't be used in callback parsers because it itself expands a macro. */
    (@as_expr @ $(:: $(@$($_:tt)* $prefixed:tt)?)? $ident:ident $(:: $path:ident)* $(($($args:tt)*))?) => (
        $crate::args_splitter!(
            [$($($prefixed)? ::)? $ident $(:: $path)*!]
            [$crate::wtflip!]
            []
            []
            [$($($args)*)?]
        );
    );

    /* Callback helper that turns @as_expr into a proper @callback invocation. */
    (@as_expr $($tokens:tt)*) => (wtflip!(@callback [] [] @as_expr $($tokens)*));

    /* @as_expr $literal callback. */
    (@callback [$($cb:tt)*] [$($args:tt)*]
        @as_expr $lit:literal $($tail:tt)*
    ) => (wtflip!(@callback [$($cb)*] [$($args)* [$lit]] $($tail)*));

    /* The compat layer but in expression position. */
    (@callback [$($cb:tt)*] [$($args:tt)*]
        @as_expr @{ $($tokens:tt)* } $($tail:tt)*
    ) => (wtflip!(@callback [$($cb)*] [$($args)* [{ $($tokens)* }]]));

    /* Convert empty callback arguments stack to an invocation of the callback with the now processed arguments. */
    (@callback [$($cb:tt)*] [$([$($arg:tt)*])*]) => ($($cb)*($($($arg)*),*));

    /* The end of a statement has been reached, process the buffer as a statement. */
    (@statement_accumulator [$($buffer:tt)*] ; $($tail:tt)*) => {
        wtflip!(@as_statement $($buffer)*);
        wtflip!($($tail)*);
    };

    /* Append all non semicolon tokens to the buffer which will be processed as a statement later. */
    (@statement_accumulator [$($buffer:tt)*] $token:tt $($tail:tt)*) => (wtflip!(
        @statement_accumulator [$($buffer)* $token] $($tail)*
    ));

    /* Finished munching the token tree */
    () => ();

    /* Just passes a non-empty token tree into a statement accumulator. */
    ($($tokens:tt)+) => (wtflip!(
        @statement_accumulator [] $($tokens)*
    ));
}

#[macro_export]
macro_rules! args_splitter {
    (
        [$($wrap:tt)*]
        [$($processor:tt)*]
        []
        [$([ $($out:tt)* ])*]
        []
    ) => ($($processor)*(@callback [$($wrap)*] [] $(@as_expr $($out)*)*));
    (
        [$($wrap:tt)*]
        [$($processor:tt)*]
        [$($current:tt)*]
        [$($out:tt)*]
        [$(, $($rest:tt)*)?]
    ) => ($crate::args_splitter!(
        [$($wrap)*]
        [$($processor)*]
        []
        [$($out)* [$($current)*]]
        [$($($rest)*)?]
    ));
    (
        [$($wrap:tt)*]
        [$($processor:tt)*]
        [$($current:tt)*]
        $out:tt
        [$no_comma:tt $($rest:tt)*]
    ) => ($crate::args_splitter!(
        [$($wrap)*]
        [$($processor)*]
        [$($current)* $no_comma]
        $out
        [$($rest)*]
    ));
}
