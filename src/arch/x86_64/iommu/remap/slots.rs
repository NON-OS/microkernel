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

//! Which interrupt remapping entries are taken: one bit per entry over the
//! 256 the table page holds. Entry 0 is never handed out, so a device left
//! with a zero handle in its MSI address hits a not-present entry and names
//! itself in a fault line rather than sharing someone's interrupt.

pub const SLOT_WORDS: usize = 4;

pub type Slots = [u64; SLOT_WORDS];

/// The lowest free entry above 0, marked taken, or `None` when all are.
pub fn take(slots: &mut Slots) -> Option<u16> {
    for (word, bits) in slots.iter_mut().enumerate() {
        let free = !*bits & if word == 0 { !1 } else { !0 };
        if free != 0 {
            let bit = free.trailing_zeros();
            *bits |= 1 << bit;
            return Some((word * 64) as u16 + bit as u16);
        }
    }
    None
}

/// Hand an entry back; `false` when it was not taken, so a double release is
/// refused instead of freeing an entry another device holds.
pub fn give(slots: &mut Slots, index: u16) -> bool {
    let (word, bit) = ((index / 64) as usize, index % 64);
    if index == 0 || word >= SLOT_WORDS || slots[word] & (1 << bit) == 0 {
        return false;
    }
    slots[word] &= !(1 << bit);
    true
}
