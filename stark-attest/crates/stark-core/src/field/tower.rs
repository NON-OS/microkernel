// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The degree-2 tower `F[X]/(X^2 - 7)` over an arbitrary `Felt` base `F`, so an
//! inner AIR's own constraint code, evaluated at `F = Ext2<G>`, computes the
//! inner extension arithmetic the way the composition check needs it.
//!
//! A recursive verifier proves the inner composition value: it recomputes each
//! inner transition from the out-of-domain frame, where every frame value is an
//! inner-`Fp2` element. The recursion holds those as base-column pairs and
//! evaluates its own constraints over `F in {Fp, Fp2}` (base composition, then
//! out-of-domain at `z`). So the inner transition must run over pairs `(F, F)`
//! with the `X^2 = 7` rule. `Ext2<F>` is exactly that pair, packaged as a `Felt`,
//! so `inner.transition::<Ext2<F>>(frame, periodic)` reproduces what the
//! hand-written join-split composition open-codes as `(a.0*b.0 + 7*a.1*b.1, ...)`
//! for any inner AIR, at no per-AIR cost. The non-residue is `W = 7`, the same as
//! `Fp2`, so `Ext2<Fp>` is that field element-for-element.

use super::element::Fp;
use super::felt::Felt;

/// An element `c0 + c1*X` of `F[X]/(X^2 - 7)`, held as its coefficient pair over
/// the base field `F`.
#[derive(Clone, Copy, PartialEq)]
pub struct Ext2<F: Felt> {
    pub c0: F,
    pub c1: F,
}

impl<F: Felt> Ext2<F> {
    /// Build from the two coefficients.
    #[inline]
    pub fn new(c0: F, c1: F) -> Ext2<F> {
        Ext2 { c0, c1 }
    }

    /// The non-residue `W = 7` lifted into the base `F`. Not a `const` because
    /// `from_base` is not a `const fn` on the trait; it is a single embed.
    #[inline]
    fn w() -> F {
        F::from_base(Fp::from_u64(7))
    }

    /// The conjugate `c0 - c1*X`.
    #[inline]
    fn conjugate(self) -> Ext2<F> {
        Ext2 { c0: self.c0, c1: F::ZERO - self.c1 }
    }

    /// The norm to the base, `N = c0^2 - W*c1^2`, zero only for the zero element.
    #[inline]
    fn norm(self) -> F {
        self.c0 * self.c0 - Self::w() * (self.c1 * self.c1)
    }
}

impl<F: Felt> core::ops::Add for Ext2<F> {
    type Output = Ext2<F>;

    #[inline]
    fn add(self, o: Ext2<F>) -> Ext2<F> {
        Ext2 { c0: self.c0 + o.c0, c1: self.c1 + o.c1 }
    }
}

impl<F: Felt> core::ops::Sub for Ext2<F> {
    type Output = Ext2<F>;

    #[inline]
    fn sub(self, o: Ext2<F>) -> Ext2<F> {
        Ext2 { c0: self.c0 - o.c0, c1: self.c1 - o.c1 }
    }
}

impl<F: Felt> core::ops::Mul for Ext2<F> {
    type Output = Ext2<F>;

    /// `(a+bX)(c+dX) = (ac + W*bd) + (ad + bc)X`, since `X^2 = W`.
    #[inline]
    fn mul(self, o: Ext2<F>) -> Ext2<F> {
        let ac = self.c0 * o.c0;
        let bd = self.c1 * o.c1;
        let ad = self.c0 * o.c1;
        let bc = self.c1 * o.c0;
        Ext2 { c0: ac + Self::w() * bd, c1: ad + bc }
    }
}

impl<F: Felt> Felt for Ext2<F> {
    const ZERO: Ext2<F> = Ext2 { c0: F::ZERO, c1: F::ZERO };
    const ONE: Ext2<F> = Ext2 { c0: F::ONE, c1: F::ZERO };

    /// Embed a base-field element as `from_base(x) + 0*X`, threading through the
    /// base `F` so a shared constant lands in the same tower level as the frame.
    #[inline]
    fn from_base(x: Fp) -> Ext2<F> {
        Ext2 { c0: F::from_base(x), c1: F::ZERO }
    }

    /// Exponentiation by square and multiply, as in every field level below.
    fn pow(self, mut exp: u64) -> Ext2<F> {
        let mut base = self;
        let mut acc = Ext2::<F>::ONE;
        while exp != 0 {
            if exp & 1 == 1 {
                acc = acc * base;
            }
            base = base * base;
            exp >>= 1;
        }
        acc
    }

    /// The inverse `(c0 - c1*X) / N`. Returns `ZERO` for `ZERO`; callers exclude
    /// it, as the base field and `Fp2` do.
    fn inv(self) -> Ext2<F> {
        if self == Ext2::<F>::ZERO {
            return Ext2::<F>::ZERO;
        }
        let n_inv = self.norm().inv();
        let conj = self.conjugate();
        Ext2 { c0: conj.c0 * n_inv, c1: conj.c1 * n_inv }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::Fp2;

    /// A deterministic base-field stream, so the check needs no rng dependency.
    fn stream(seed: u64) -> impl FnMut() -> Fp {
        let mut s = seed;
        move || {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            Fp::from_u64(s ^ (s >> 29))
        }
    }

    fn to_fp2(a: Ext2<Fp>) -> Fp2 {
        Fp2::new(a.c0, a.c1)
    }

    /// `Ext2<Fp>` is `Fp[X]/(X^2-7)` element-for-element, so every operation must
    /// agree with `Fp2`. This pins the tower arithmetic to the trusted `Fp2` the
    /// prover and verifier already share, before any AIR is evaluated over it.
    #[test]
    fn ext2_over_fp_matches_fp2() {
        let mut next = stream(0x9E3779B97F4A7C15);
        for _ in 0..2000 {
            let (a0, a1, b0, b1) = (next(), next(), next(), next());
            let a = Ext2::<Fp>::new(a0, a1);
            let b = Ext2::<Fp>::new(b0, b1);
            let fa = Fp2::new(a0, a1);
            let fb = Fp2::new(b0, b1);

            assert_eq!(to_fp2(a + b), fa + fb, "add");
            assert_eq!(to_fp2(a - b), fa - fb, "sub");
            assert_eq!(to_fp2(a * b), fa * fb, "mul");
            assert_eq!(to_fp2(a.pow(37)), fa.pow(37), "pow");
            assert_eq!(to_fp2(Ext2::<Fp>::from_base(a0)), Fp2::from_base(a0), "from_base");
            if fa != Fp2::ZERO {
                assert_eq!(to_fp2(a.inv()), fa.inv(), "inv");
                assert_eq!(to_fp2(a * a.inv()), Fp2::ONE, "inv is a right inverse");
            }
        }
    }

    /// The non-residue is a non-square: `X` itself squares to the base `7`, which
    /// is the whole reason the quotient is a field.
    #[test]
    fn x_squared_is_seven() {
        let x = Ext2::<Fp>::new(Fp::ZERO, Fp::ONE);
        assert_eq!(to_fp2(x * x), Fp2::from_base(Fp::from_u64(7)));
    }
}
