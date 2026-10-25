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

//! The remapping structures after DMAR's header (L70), each a whole one
//! inside the table. Pure, over the table's bytes, so the host proofs feed it
//! hostile tables; the kernel had walked them by pointer to whatever length a
//! structure named.

use super::dmar_rmrr::RMRR_TYPE;

/// A 36-byte SDT header, the host address width, flags and ten reserved bytes.
pub const FIRST_STRUCTURE: usize = 48;

/// The kinds firmware lists after the RMRRs: ATSR, RHSA, ANDD and SATC, in
/// that order (VT-d's DMAR layout, recalled, unverified).
const AFTER_RMRR: core::ops::RangeInclusive<u16> = 2..=5;

/// How a walk ended: cut short at a structure under four bytes or running past
/// the table, and whether a kind listed after the RMRRs came before that, so
/// the cut hid no RMRR.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Walked {
    pub cut_short: bool,
    pub past_rmrr: bool,
}

/// `each(kind, structure)` for every whole structure in order, until one is
/// not whole. Fewer than four bytes left, or only zeros (firmware pads a table
/// out to its length), is the table's end.
pub fn walk_structures(table: &[u8], mut each: impl FnMut(u16, &[u8])) -> Walked {
    let (mut at, mut past_rmrr) = (FIRST_STRUCTURE, false);
    while let Some(head) = table.get(at..at + 4) {
        let kind = u16::from_le_bytes([head[0], head[1]]);
        let length = u16::from_le_bytes([head[2], head[3]]) as usize;
        let Some(whole) = table.get(at..at + length).filter(|_| length >= 4) else {
            let padding = table[at..].iter().all(|&b| b == 0);
            return Walked { cut_short: !padding, past_rmrr };
        };
        past_rmrr |= kind != RMRR_TYPE && AFTER_RMRR.contains(&kind);
        each(kind, whole);
        at += length;
    }
    Walked { cut_short: false, past_rmrr }
}
