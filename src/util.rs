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
        ([$$([$$($$segment:tt)*])*] [$$($$tt:tt $$($$buffer:tt)*)?] [$$(_)?]) => ({
            $callback!($([$($carry)*])? $$([$$($$segment)*])* $$([$$tt $$($$buffer)*])?)
        });
    }

    __segments_internal_parser!([] [] [] $($tokens)*)
})}

#[macro_export]
macro_rules! __map {([[$callback:path $(: $($args:tt)*)?] $([$($carry:tt)*])?] $([$($tokens:tt)*])*) => ({
    macro_rules! __mapper {
        ([$$($$output:tt)*] [$$($$item:tt)*] $$($$tail:tt)*) => (__mapper!(
            [$$($$output)* [@$callback!($([$($carry)*])? $($($args)*)? $$($$item)*)]] $$($$tail)*
        ));
        ([$$([$$($$output:tt)*])*]) => ($crate::defile!([$$($$($$output)*),*]));
    }

    __mapper!([] $([$($tokens)*])*)
})}

#[macro_export]
macro_rules! __optional {
    ([$map:path $([$($carry:tt)*])?] $($tokens:tt)*) => (::core::option::Option::Some(
        $crate::defile!(@$map!($([$($carry)*])? $($tokens)*)),
    ));
    ([$map:path $([$($carry:tt)*])?]) => (::core::option::Option::None);
}
