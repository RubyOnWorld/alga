// Copyright 2013-2014 The Num-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ops::{Op, Additive, Multiplicative};

use structure::SemigroupApprox;
use structure::Semigroup;
use structure::{Identity, id};

/// A type that is equipped with an approximately associative operator
/// and a corresponding identity. This should satisfy:
///
/// ~~~notrust
/// a + e ≈ a       ∀ a ∈ Self
/// e + a ≈ a       ∀ a ∈ Self
/// ~~~
pub trait MonoidApprox<O: Op>
    : SemigroupApprox<O>
    + Identity<O>
{
    /// Checks whether operating with identity is approximately a no-op for the given
    /// argument.
    fn prop_operating_identity_is_noop_approx(a: Self) -> bool {
        let a = || a.clone();
        a().approx(id()) == a() &&
        Self::approx_eq(&Self::id().approx(a()), &a())
    }
}

impl_marker!(MonoidApprox<Additive>; u8, u16, u32, u64, i8, i16, i32, i64,);
impl_marker!(MonoidApprox<Multiplicative>; u8, u16, u32, u64, i8, i16, i32, i64,);

/// A type that is equipped with an associative operator and a
/// corresponding identity. This should satisfy:
///
/// ~~~notrust
/// a + e = a                           ∀ a ∈ Self
/// e + a = a                           ∀ a ∈ Self
/// ~~~
pub trait Monoid<O: Op>
    : MonoidApprox<O>
    + Semigroup<O>
{
    /// Checks whether operating with identity is a no-op for the given argument.
    fn prop_operating_identity_is_noop(a: Self) -> bool {
        let a = || a.clone();
        a().operate(id()) == a() &&
        Self::id().operate(a()) == a()
    }
}

impl_marker!(Monoid<Additive>; u8, u16, u32, u64, i8, i16, i32, i64,);
impl_marker!(Monoid<Multiplicative>; u8, u16, u32, u64, i8, i16, i32, i64,);

#[cfg(test)]
mod tests {
    macro_rules! check_int {
        ($T:ident) => {
            mod $T {
                use structure::MonoidAdditiveApprox;
                use structure::MonoidAdditive;
                use structure::MonoidMultiplicativeApprox;
                use structure::MonoidMultiplicative;

                #[quickcheck]
                fn prop_add_zero_is_noop_approx(args: $T) -> bool {
                    MonoidAdditiveApprox::prop_add_zero_is_noop_approx(args)
                }
                #[quickcheck]
                fn prop_add_zero_is_noop(args: $T) -> bool {
                    MonoidAdditive::prop_add_zero_is_noop(args)
                }

                #[quickcheck]
                fn prop_mul_unit_is_noop_approx(args: $T) -> bool {
                    MonoidMultiplicativeApprox::prop_mul_unit_is_noop_approx(args)
                }
                #[quickcheck]
                fn prop_mul_unit_is_noop(args: $T) -> bool {
                    MonoidMultiplicative::prop_mul_unit_is_noop(args)
                }
            }
        }
    }
    check_int!(u8);
    check_int!(u16);
    check_int!(u32);
    check_int!(u64);
    check_int!(i8);
    check_int!(i16);
    check_int!(i32);
    check_int!(i64);
}
