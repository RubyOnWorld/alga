//! Wrappers that attach an algebraic structure with a value type.

use std::ops::{Add, Neg, Sub, Mul, Div};
use std::fmt::{Display, Formatter, Error};

use general::{Op, Inverse, Recip, Additive, Identity, Multiplicative};
use numeric::ApproxEq;

use general::Magma;
use general::Quasigroup;

/// Wrapper that allows to use operators on algebraic types.
#[derive(Clone, Copy, PartialOrd, PartialEq, Debug)]
pub struct Wrapper<M>(pub M);

impl<M: Display> Display for Wrapper<M> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        self.0.fmt(fmt)
    }
}

/// Creates wrapper with identity value for a specific operator.
pub fn id<O: Op, M>(_: O) -> Wrapper<M>
where M: Identity<O>
{
    Wrapper(Identity::<O>::id())
}

impl<M> ApproxEq for Wrapper<M>
where M: ApproxEq
{
    type Epsilon = M::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        M::default_epsilon()
    }

    fn default_max_relative() -> Self::Epsilon {
        M::default_max_relative()
    }

    fn default_max_ulps() -> u32 {
        M::default_max_ulps()
    }

    fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
        self.0.relative_eq(&other.0, epsilon, max_relative)
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.0.ulps_eq(&other.0, epsilon, max_ulps)
    }
}

impl<M> Add<Wrapper<M>> for Wrapper<M>
where M: Magma<Additive>
{
    type Output = Self;
    fn add(self, lhs: Self) -> Self {
        Wrapper(self.0.operate(lhs.0))
    }
}

impl<M> Neg for Wrapper<M>
where M: Quasigroup<Additive>
{
    type Output = Self;
    fn neg(mut self) -> Self {
        self.0 = self.0.inv();
        self
    }
}

impl<M> Sub<Wrapper<M>> for Wrapper<M>
where M: Quasigroup<Additive>
{
    type Output = Self;
    fn sub(self, lhs: Self) -> Self {
        self + -lhs
    }
}

impl<M> Mul<Wrapper<M>> for Wrapper<M>
where M: Magma<Multiplicative>
{
    type Output = Self;
    fn mul(self, lhs: Self) -> Self {
        Wrapper(self.0.operate(lhs.0))
    }
}

impl<M> Recip for Wrapper<M>
where M: Quasigroup<Multiplicative>
{
    type Result = Self;
    fn recip(self) -> Self {
        Wrapper(self.0.inv())
    }
}

impl<M> Div<Wrapper<M>> for Wrapper<M>
where M: Quasigroup<Multiplicative>
{
    type Output = Self;
    fn div(self, lhs: Self) -> Self {
        self * lhs.inv()
    }
}
