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
//! A DPLL's frequency as DPLL_CFGCR0 and DPLL_CFGCR1 hold it.

/// The DCO as a multiple of the reference (an integer, and a fraction in
/// 2^-15 steps), and the post dividers P, Q and K.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dividers {
    pub integer: u32,
    pub fraction: u32,
    pub p: u32,
    pub q: u32,
    pub k: u32,
}

impl Dividers {
    /// DPLL_CFGCR0: DCO Fraction 24:10, DCO Integer 9:0 (Vol 2c p. 650);
    /// None when either does not fit its field.
    pub fn cfgcr0(&self) -> Option<u32> {
        (self.integer < 1 << 10 && self.fraction < 1 << 15)
            .then_some((self.fraction << 10) | self.integer)
    }

    /// DPLL_CFGCR1 (p. 651, 652): Qdiv Ratio 17:10 with Qdiv Mode 9 set
    /// when Q is not 1; Kdiv 8:6 as 001b, 010b, 100b for 1, 2, 3; Pdiv 5:2
    /// as 0001b, 0010b, 0100b, 1000b for 2, 3, 5, 7; cfselovrd 1:0 left at
    /// 00b, the normal crystal. None for a divider the fields cannot hold,
    /// or a Q other than 1 with K other than 2, which the page forbids.
    pub fn cfgcr1(&self) -> Option<u32> {
        let k = match self.k {
            1 => 0b001,
            2 => 0b010,
            3 => 0b100,
            _ => return None,
        };
        let p = match self.p {
            2 => 0b0001,
            3 => 0b0010,
            5 => 0b0100,
            7 => 0b1000,
            _ => return None,
        };
        let q = match self.q {
            1 => 0,
            2..=255 if self.k == 2 => (self.q << 10) | (1 << 9),
            _ => return None,
        };
        Some(q | (k << 6) | (p << 2))
    }
}
