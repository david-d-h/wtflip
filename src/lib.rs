#[macro_export]
macro_rules! wtflip {
    (return $($tail:tt)*) => (wtflip!( // return statement
        @internal_return_accumulator [] $($tail)*
    ));
    (# $ident:ident $(: $(@$($_:tt)* $declare:tt)?)?= $($expr:tt)+) => { // variable declaration or assignment
        wtflip!(@internal_build_expr [$($($declare)? #[allow(unused_mut)] let mut)? $ident =] [] $($expr)+);
    };
    (if ($($cond:tt)*) { $($block:tt)* } $($tail:tt)*) => (wtflip!( // if, many optional elif and optional else statement
        @internal_conditional_clause_accumulator [if wtflip!($($cond)*) { wtflip!($($block)*); }] $($tail)*)
    );
    ( // macro invocation
        @ $(:: $(@$($_:tt)? $prefix:tt)?)? $ident:ident $(:: $path:ident)* $(($($args:tt)*))? // get the macro and raw args
        $(; $(@$($__:tt)* $statement:tt)? $($tail:tt)*)? // determine if it's a statement
    ) => {
        wtflip!(@split_tts [$($($prefix)? ::)? $ident $(:: $path)*!] $($($args)*)?) $(; $($statement)? wtflip!($($tail)*))?
    };
    ($lit:literal) => { // literal
        $lit
    };
    (# $ident:ident) => { // identifier
        $ident
    };
    (@internal_build_expr [$($tokens:tt)*] [$($buffer:tt)*] ; $($tail:tt)*) => {
        // This accumulator puts the already assembled `$tokens` buffer next to the now completed expression `$buffer`.
        // This results in a complete variable declaration. It carries the remaining tokens `$tail` and passes them
        // to the macro again (TT munching).
        $($tokens)* wtflip!($($buffer)*);
        wtflip!($($tail)*);
    };
    (@internal_build_expr [$($tokens:tt)*] [$($buffer:tt)*] $x:tt $($tail:tt)*) => {
        // This accumulator receives a buffer of tokens that is the part until the `=` sign,
        //`$x` and `$tail` are the expression tokens, depending on `$x` being `;` it calls the accumulator above.
        // If `$x` is not `;` `$x` will simply be added to the (expression) `$buffer`.
        wtflip!(@internal_build_expr [$($tokens)*] [$($buffer)* $x] $($tail)*);
    };
    (@internal_return_accumulator [$($buffer:tt)*] ; $($tail:tt)*) => {
        return wtflip!($($buffer)*);
        wtflip!($($tail)*);
    };
    (@internal_return_accumulator [$($buffer:tt)*] $x:tt $($tail:tt)*) => (wtflip!(
        @internal_return_accumulator [$($buffer)* $x] $($tail)*
    ));
    (@internal_conditional_clause_accumulator [$($tokens:tt)*] else if ($($cond:tt)*) { $($block:tt)* } $($tail:tt)*) => (wtflip!(
        @internal_conditional_clause_accumulator [$($tokens)*
            else if wtflip!($($cond)*) { wtflip!($($block)*) }
        ] $($tail)*
    ));
    (@internal_conditional_clause_accumulator [$($tokens:tt)*] else { $($block:tt)* } $($tail:tt)*) => {
        $($tokens)* else { wtflip!($($block)*); }
        wtflip!($($tail)*);
    };
    (@internal_conditional_clause_accumulator [$($tokens:tt)*] $($tail:tt)*) => {
        $($tokens)* wtflip!($($tail)*);
    };
    (@split_tts [$($wrap:tt)*] $($input:tt)*) => (wtflip!(@internal_split_tts
        [$($wrap)*]
        []
        []
        [$($input)*]
    ));
    (@internal_split_tts
        [$($wrap:tt)*]
        []
        [$([ $($out:tt)* ])*]
        []
    ) => {
        $($wrap)*($(wtflip!($($out)*),)*)
    };
    (@internal_split_tts
        [$($wrap:tt)*]
        [$($current:tt)*]
        [$($out:tt)*]
        [$(, $($rest:tt)*)?]
    ) => (wtflip!(@internal_split_tts
        [$($wrap)*]
        []
        [$($out)* [$($current)*]]
        [$($($rest)*)?]
    ));
    (@internal_split_tts
        [$($wrap:tt)*]
        [$($current:tt)*]
        $out:tt
        [$not_a_comma:tt $($rest:tt)*]
    ) => (wtflip!(@internal_split_tts
        [$($wrap)*]
        [$($current)* $not_a_comma]
        $out
        [$($rest)*]
    ));
    ({$($tokens:tt)*}) => { $($tokens)* };
    () => {};
}
