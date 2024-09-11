#[macro_export]
macro_rules! segments {($callback:path $([$($carry:tt)*])? where [$($match:tt)*] in $($tokens:tt)*) => ({
    macro_rules! __segments_internal_parser {
        ([$$([$$($$segment:tt)*])*] [/* buffer is empty, skip */] [] $($match)* $$($$tail:tt)*) => (__segments_internal_parser!(
            [$$([$$($$segment)*])*] [] [_] $$($$tail)*
        ));
        ([$$([$$($$segment:tt)*])*] [$$($$buffer:tt)+] [/* whether a segment is expected */] $($match)* $$($$tail:tt)*) => (__segments_internal_parser!(
            [$$([$$($$segment)*])* [$$($$buffer)+]] [] [_] $$($$tail)*
        ));
        ([$$([$$($$segment:tt)*])*] [$$($$buffer:tt)*] [_] $($match)* $$($$tail:tt)*) => (::core::compile_error!(
            ::core::concat!("expected segment, found (at least two) consequential segment splitters (", ::core::stringify!($($match)*), ")"),
        ));
        ([$$([$$($$segment:tt)*])*] [$$($$buffer:tt)*] [$$(_)?] $$tt:tt $$($$tail:tt)*) => (__segments_internal_parser!(
            [$$([$$($$segment)*])* ] [$$($$buffer)* $$tt] [] $$($$tail)*
        ));
        ([$$([$$($$segment:tt)*])*] [$$($$tt:tt $$(@$$($$_:tt)* $$has_buffer:tt)? $$($$buffer:tt)*)?] [$$(_)?]) => ({
            $callback!($([$($carry)*])? $$([$$($$segment)*])* $$($$($$has_buffer)? [$$tt $$($$buffer)*])?)
        });
    }

    __segments_internal_parser!([] [] [] $($tokens)*)
})}