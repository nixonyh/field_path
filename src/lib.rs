#![doc = include_str!("../README.md")]
#![no_std]

pub mod field;
pub mod lens;
pub mod path;

// Splits `Source.field...` at the first top-level `.` and calls
// `$crate::$callback!(@build [Source] .field...)`. Turbofish is
// required so the call stays valid Rust for rustfmt.
#[doc(hidden)]
#[macro_export]
macro_rules! __split_source {
    (@split $callback:ident; [$($source:tt)+] $(.$field:tt)*) => {
        $crate::$callback!(@build [$($source)+] $(.$field)*)
    };
    (@split $callback:ident; [$($source:tt)*] :: < $($rest:tt)*) => {
        $crate::__split_source!(@generics $callback; [$($source)* :: <] $($rest)*)
    };
    (@split $callback:ident; [$($source:tt)*] < $($rest:tt)*) => {
        compile_error!("generic arguments must use turbofish: `Type::<T>`")
    };
    (@split $callback:ident; [$($source:tt)*] $next:tt $($rest:tt)*) => {
        $crate::__split_source!(@split $callback; [$($source)* $next] $($rest)*)
    };
    (@generics $callback:ident; [$($source:tt)+] $(.$field:tt)*) => {
        $crate::$callback!(@build [$($source)+] $(.$field)*)
    };
    (@generics $callback:ident; [$($source:tt)*] $next:tt $($rest:tt)*) => {
        $crate::__split_source!(@generics $callback; [$($source)* $next] $($rest)*)
    };
    ($callback:ident; $($input:tt)+) => {
        $crate::__split_source!(@split $callback; [] $($input)+)
    };
}
