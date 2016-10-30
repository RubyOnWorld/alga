/// Nested sets and conversions between them (using an injective mapping). Useful to work with
/// substructures. In generic code, it is preferable to use `SupersetOf` as trait bound whenever
/// possible instead of `SubsetOf` (because SupersetOf is automatically implemented whenever
/// `SubsetOf` is.
///
/// The notion of "nested sets" is very broad and applies to what the types are _supposed to
/// represent_, independently from their actual implementation details and limitations. For
/// example:
/// * f32 and f64 are both supposed to represent reals and are thus considered equal (even if in
/// practice f64 has more elements).
/// * u32 and i8 are respectively supposed to represent natural and relative numbers. Thus, u32 is
/// a subset of i8.
/// * A quaterion and a 3x3 orthogonal matrix with unit determinant are both sets of rotations.
/// They can thus be considered equal.
///
/// In other words, implementation details due to machine limitations are ignored (otherwise we
/// could not even, e.g., convert a u64 to an i64). If considering those limintations are
/// important, other crates allowing you to query the limitations of given types should be used.
pub trait SubsetOf<T>: Sized {
    /// The inclusion map: converts `self` to the equivalent element of its superset.
    fn to_superset(&self) -> T;

    /// The inverse inclusion map: attempts to construct `self` from the equivalent element of its
    /// superset.
    ///
    /// Must return `None` if `element` has no equivalent in `Self`.
    fn from_superset(element: &T) -> Option<Self> {
        if Self::is_in_subset(element) {
            Some(unsafe { Self::from_superset_unchecked(element) })
        }
        else {
            None
        }
    }

    /// Use with care! Same as `self.to_superset` but without any property checks. Always succeeds.
    unsafe fn from_superset_unchecked(element: &T) -> Self;

    /// Checks if `element` is actually part of the subset `Self` (and can be converted to it).
    fn is_in_subset(element: &T) -> bool;
}

/// Nested sets and conversions between them. Useful to work with substructures. It is preferable
/// to implement the `SupersetOf` trait instead of `SubsetOf` whenever possible (because
/// `SupersetOf` is automatically implemented whenever `SubsetOf` is.
///
/// The notion of "nested sets" is very broad and applies to what the types are _supposed to
/// represent_, independently from their actual implementation details and limitations. For
/// example:
/// * f32 and f64 are both supposed to represent reals and are thus considered equal (even if in
/// practice f64 has more elements).
/// * u32 and i8 are respectively supposed to represent natural and relative numbers. Thus, i8 is
/// a superset of u32.
/// * A quaterion and a 3x3 orthogonal matrix with unit determinant are both sets of rotations.
/// They can thus be considered equal.
///
/// In other words, implementation details due to machine limitations are ignored (otherwise we
/// could not even, e.g., convert a u64 to an i64). If considering those limintations are
/// important, other crates allowing you to query the limitations of given types should be used.
pub trait SupersetOf<T>: Sized {
    /// The inverse inclusion map: attempts to construct `self` from the equivalent element of its
    /// superset.
    ///
    /// Must return `None` if `element` has no equivalent in `Self`.
    fn to_subset(&self) -> Option<T> {
        if self.is_in_subset() {
            Some(unsafe { self.to_subset_unchecked() })
        }
        else {
            None
        }
    }

    /// Checks if `self` is actually part of its subset `T` (and can be converted to it).
    fn is_in_subset(&self) -> bool;

    /// Use with care! Same as `self.to_subset` but without any property checks. Always succeeds.
    unsafe fn to_subset_unchecked(&self) -> T;

    /// The inclusion map: converts `self` to the equivalent element of its superset.
    fn from_subset(element: &T) -> Self;
}

impl<SS: SubsetOf<SP>, SP> SupersetOf<SS> for SP {
    #[inline]
    fn to_subset(&self) -> Option<SS> {
        SS::from_superset(self)
    }

    #[inline]
    fn is_in_subset(&self) -> bool {
        SS::is_in_subset(self)
    }

    #[inline]
    unsafe fn to_subset_unchecked(&self) -> SS {
        SS::from_superset_unchecked(self)
    }

    #[inline]
    fn from_subset(element: &SS) -> Self {
        element.to_superset()
    }
}

macro_rules! impl_subset(
    ($($subset: ty as $( $superset: ty),+ );* $(;)*) => {
        $($(
        impl SubsetOf<$superset> for $subset {
            #[inline]
            fn to_superset(&self) -> $superset {
                *self as $superset
            }

            #[inline]
            unsafe fn from_superset_unchecked(element: &$superset) -> $subset {
                *element as $subset
            }

            #[inline]
            fn is_in_subset(_: &$superset) -> bool {
                true
            }
        }
        )+)*
    }
);

impl_subset!(
    u8    as u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64;
    u16   as u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64;
    u32   as u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64;
    u64   as u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64;
    usize as u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64;

    i8    as i8, i16, i32, i64, isize, f32, f64;
    i16   as i8, i16, i32, i64, isize, f32, f64;
    i32   as i8, i16, i32, i64, isize, f32, f64;
    i64   as i8, i16, i32, i64, isize, f32, f64;
    isize as i8, i16, i32, i64, isize, f32, f64;

    f32 as f32, f64;
    f64 as f32, f64;
);
