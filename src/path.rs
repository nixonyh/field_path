//! This module defines the [`Path`] type, which pairs a
//! [`Field`] (representing the static path name) with an
//! [`Lens`] (providing functional pointers for data access).
//!
//! Use the [`path!`] macro to ensure that the path name and
//! the access logic are always synchronized to the same field.

use core::hash::{Hash, Hasher};

use crate::field::Field;
use crate::lens::Lens;
// For docs.
#[expect(unused_imports)]
use crate::path;

/// A specialized container pairing a [`Field`] with its [`Lens`].
#[derive(Debug)]
pub struct Path<S, T> {
    pub field: Field<S, T>,
    pub lens: Lens<S, T>,
}

impl<S, T> Path<S, T> {
    #[inline]
    pub const fn new(field: Field<S, T>, lens: Lens<S, T>) -> Self {
        Self { field, lens }
    }
}

impl<S, T> Hash for Path<S, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.field.hash(state);
    }
}

impl<S, T> PartialEq for Path<S, T> {
    fn eq(&self, other: &Self) -> bool {
        self.field.eq(&other.field)
    }
}

impl<S, T> Eq for Path<S, T> {}

impl<S, T> Clone for Path<S, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S, T> Copy for Path<S, T> {}

/// Creates a [`Path`] that ensures both [`Field`] and
/// [`Lens`] are pointing to the same field path.
///
/// ## Example
///
/// ```
/// use field_path::path;
/// use field_path::path::Path;
///
/// struct Foo {
///     value: i32,
/// }
///
/// const FOO_FIELD_ACC: Path<Foo, i32> = path!(Foo.value);
///
/// assert_eq!(FOO_FIELD_ACC.field.field_path(), ".value");
///
/// let mut foo = Foo { value: 42 };
///
/// assert_eq!(FOO_FIELD_ACC.lens.get_ref(&foo), &42);
/// *FOO_FIELD_ACC.lens.get_mut(&mut foo) = 999;
/// assert_eq!(foo.value, 999);
/// ```
///
/// Generic and qualified source types work as well, e.g.
/// `path!(Vec2<f32>.x)` or `path!(std::ops::Range<u32>.start)`.
///
/// Nested tuple indices such as `.0.1` are lexed as a float by Rust,
/// so they are not supported.
#[macro_export]
macro_rules! path {
    ($($input:tt)+) => {
        $crate::path::Path::new(
            $crate::field!($($input)+),
            $crate::lens!($($input)+),
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Vec2<T> {
        x: T,
    }
    struct Transform {
        translation: Vec2<f32>,
    }
    #[allow(dead_code)]
    struct Pair(u32, u32);

    #[test]
    fn dot_syntax() {
        const X: Path<Transform, f32> =
            path!(Transform.translation.x);
        const GENERIC: Path<Vec2<f32>, f32> = path!(Vec2<f32>.x);
        const TUPLE: Path<Pair, u32> = path!(Pair.1);
        assert_eq!(X.field.field_path(), ".translation.x");
        const QUALIFIED: Path<core::ops::Range<u32>, u32> =
            path!(core::ops::Range<u32>.start);
        assert_eq!(QUALIFIED.field.field_path(), ".start");
        assert_eq!(GENERIC.field.field_path(), ".x");
        assert_eq!(TUPLE.field.field_path(), ".1");
    }

    // Deliberately badly spaced; `cargo fmt --check` must normalize
    // it.
    #[test]
    fn rustfmt_ugly() {
        let _: Path<Transform, f32> = path!(Transform.translation.x);
    }
}
