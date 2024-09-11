#[allow(non_snake_case)]
macro_rules! VarDeclaration {({ $($tokens:tt)* }) => ({
    $crate::segments!(
        $crate::testing::__construct_from_fields [$crate::statement::VarDeclaration]
        where [,] in $($tokens)*
    )
})} pub(crate) use VarDeclaration;

#[allow(non_snake_case)]
macro_rules! VarAssignment {({ $($tokens:tt)* }) => ({
    $crate::segments!(
        $crate::testing::__construct_from_fields [$crate::statement::VarAssignment]
        where [,] in $($tokens)*
    )
})} pub(crate) use VarAssignment;

#[allow(non_snake_case)]
macro_rules! Statement {
    ($variant:ident $tt:tt) => ({
        $crate::statement::Statement::$variant(
            $crate::testing::$variant!($tt),
        )
    });
    ($variant:ident) => ({
        ::core::compile_error!(
            ::core::concat!("missing grouped tts after Statement::", ::core::stringify!($variant))
        );
        // analysis and auto-completion
        $crate::statement::Statement::$variant;
    });
} pub(crate) use Statement;