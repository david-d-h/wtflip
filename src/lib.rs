#[macro_export]
macro_rules! wtflip {
    /* Mutable declaration */
    (mut $ident:ident := $($expr:tt)+) => (wtflip!(
        @as_expr_assign [let mut $ident =] [] $($expr)+
    ));

    /* Assignment or immutable declaration. */
    ($ident:ident $(: $(@$($_:tt)* $declaration:tt)?)?= $($expr:tt)+) => (wtflip!(
        @as_expr_assign [$($($declaration)? let)? $ident =] [] $($expr)+
    ));

    /*
        Macro invocation.
        The macro invocation can't be used in callback parsers because it itself expands a macro.
     */
    (@as_expr @ $(:: $(@$($_:tt)* $prefixed:tt)?)? $ident:ident $(:: $path:ident)* $(($($args:tt)*))?) => (
        $crate::args_splitter!(
            [$($($prefixed)? ::)? $ident $(:: $path)*!]
            [$crate::wtflip!]
            []
            []
            [$($($args)*)?]
        );
    );

    /* Callback helper that turns @as_expr into a proper @callback invocation */
    (@as_expr $($tokens:tt)*) => (wtflip!(@callback [] [] @as_expr $($tokens)*));

    /* @as_expr $literal callback */
    (@callback [$($cb:tt)*] [$($args:tt)*]
        @as_expr $lit:literal $($tail:tt)*
    ) => (wtflip!(@callback [$($cb)*] [$($args)* [$lit]] $($tail)*));

    /* Convert empty callback arguments stack to an invocation of the callback with the now processed arguments */
    (@callback [$($cb:tt)*] [$([$($arg:tt)*])*]) => ($($cb)*($($($arg)*),*));

    /* Assignment (expression) accumulator */
    (@as_expr_assign [$($tokens:tt)*] [$($expr:tt)*] ; $($tail:tt)*) => {
        $($tokens)* wtflip!(@as_expr $($expr)*);
        wtflip!($($tail)*);
    };
    (@as_expr_assign [$($tokens:tt)*] [$($buffer:tt)*] $append:tt $($expr:tt)*) => (wtflip!(
        @as_expr_assign [$($tokens)*] [$($buffer)* $append] $($expr)*
    ));
    /*
        A compatibility layer.
        Allows for writing Rust in the wtflip invocation.
    */
    ({ $($compat:tt)* }) => ($($compat)*);
    () => {};
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
