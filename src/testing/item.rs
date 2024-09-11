#[allow(non_snake_case)]
macro_rules! Item {
    ($variant:ident $tt:tt) => (
        $crate::item::Item::$variant(
            $crate::testing::item::$variant!($tt),
        )
    );
    ($variant:ident) => ({
        ::core::compile_error!(
            ::core::concat!("missing grouped tts after Item::", ::core::stringify!($variant))
        );
        $crate::item::Item::$variant;
    });
}