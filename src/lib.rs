#[macro_export]
macro_rules! wtflip {
    (# $ident:ident $(: $(@$($_:tt)* $declare:tt)?)?= $($expr:tt)+) => { // variable declaration or assignment
        wtflip!(@internal_build_expr[$($($declare)? #[allow(unused_mut)] let mut)? $ident =] [] $($expr)+);
    };
    (
        if ($($cond:tt)*) { $($block:tt)* } // if statement
        $(elseif ($($elif_cond:tt)*) { $($elif_block:tt)* })* // many elif statements
        else { $($else_block:tt)* } // else statement
        $($tail:tt)* // tail
    ) => { // if, else and (many) elif statements
        if wtflip!($($cond)*) { wtflip!($($block)*); }
        $(else if ($elif_cond) { wtflip!($($elif_block)*); })*
        else { wtflip!($($else_block)*) }
        wtflip!($($tail)*);
    };
    (
        if ($($cond:tt)*) { $($block:tt)* } // if statement
        $($tail:tt)* // tail
    ) => { // if statement
        if wtflip!($($cond)*) { wtflip!($($block)*); }
        wtflip!($($tail)*);
    };
    ($lit:literal) => { // literal
        $lit
    };
    (# $ident:ident) => { // identifier
        $ident
    };
    (@internal_build_expr[$($tokens:tt)*] [$($buffer:tt)*] ; $($tail:tt)*) => {
        // This accumulator puts the already assembled `$tokens` buffer next to the now completed expression `$buffer`.
        // This results in a complete variable declaration. It carries the remaining tokens `$tail` and passes them
        // to the macro again (TT munching).
        $($tokens)* wtflip!($($buffer)*);
        wtflip!($($tail)*);
    };
    (@internal_build_expr[$($tokens:tt)*] [$($buffer:tt)*] $x:tt $($tail:tt)*) => {
        // This accumulator receives a buffer of tokens that is the part until the `=` sign,
        //`$x` and `$tail` are the expression tokens, depending on `$x` being `;` it calls the accumulator above.
        // If `$x` is not `;` `$x` will simply be added to the (expression) `$buffer`.
        wtflip!(@internal_build_expr[$($tokens)*] [$($buffer)* $x] $($tail)*);
    };
    () => { // Finished TT munching.
        //
    };
}
