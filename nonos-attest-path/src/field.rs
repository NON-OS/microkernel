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

/// The Goldilocks modulus, 2^64 - 2^32 + 1.
pub(crate) const P: u64 = 0xFFFF_FFFF_0000_0001;

/// A field element, always held canonical in `[0, P)`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Fp(u64);

impl Fp {
    pub const ZERO: Fp = Fp(0);

    /// Reduce any word. One subtraction suffices because `2^64 - P < P`.
    pub const fn from_u64(x: u64) -> Fp {
        Fp(if x >= P { x - P } else { x })
    }

    /// The element a word names only when the word is already canonical, so an
    /// encoding has one reading and a non-canonical one is refused, not reduced.
    pub const fn canonical(x: u64) -> Option<Fp> {
        if x < P {
            Some(Fp(x))
        } else {
            None
        }
    }

    pub const fn value(self) -> u64 {
        self.0
    }

    pub(crate) fn add(self, o: Fp) -> Fp {
        Fp((((self.0 as u128) + (o.0 as u128)) % (P as u128)) as u64)
    }

    pub(crate) fn sub(self, o: Fp) -> Fp {
        Fp((((self.0 as u128) + (P as u128) - (o.0 as u128)) % (P as u128)) as u64)
    }

    pub(crate) fn mul(self, o: Fp) -> Fp {
        Fp((((self.0 as u128) * (o.0 as u128)) % (P as u128)) as u64)
    }

    pub(crate) fn pow(self, mut e: u64) -> Fp {
        let (mut base, mut acc) = (self, Fp(1));
        while e != 0 {
            if e & 1 == 1 {
                acc = acc.mul(base);
            }
            base = base.mul(base);
            e >>= 1;
        }
        acc
    }

    /// `a^(p-2)`. Only ever called on the Cauchy denominators, none of which is
    /// zero, since the two node sets are disjoint.
    pub(crate) fn inv(self) -> Fp {
        self.pow(P - 2)
    }
}
