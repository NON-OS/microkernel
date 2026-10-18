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

//! BONDS's reply, one entry a bond, in the list's order.

use super::list::{entries, BONDS_MAX, BOND_LEN};

/// One bond: its identity (address type 0 public or 1 static random, and
/// the address least significant octet first), whether pairing was
/// authenticated, and whether the link runs on it. Never a key.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Bond {
    pub kind: u8,
    pub addr: [u8; 6],
    pub authenticated: bool,
    pub linked: bool,
}

/// A checked BONDS payload.
#[derive(Clone, Copy, Debug)]
pub struct Bonds<'a>(&'a [u8]);

impl<'a> Bonds<'a> {
    /// Each flag is 0 or 1 and each address type 0 or 1, or it is no list.
    pub fn read(data: &'a [u8]) -> Option<Self> {
        let rest = entries(data, BOND_LEN, BONDS_MAX)?;
        let good = |e: &[u8]| e[0] <= 1 && e[7] <= 1 && e[8] <= 1;
        rest.chunks_exact(BOND_LEN).all(good).then_some(Self(rest))
    }

    pub fn len(&self) -> usize {
        self.0.len() / BOND_LEN
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, i: usize) -> Option<Bond> {
        let e = self.0.chunks_exact(BOND_LEN).nth(i)?;
        let addr = [e[1], e[2], e[3], e[4], e[5], e[6]];
        Some(Bond { kind: e[0], addr, authenticated: e[7] == 1, linked: e[8] == 1 })
    }
}
