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

//! Which entries of a device's remapping table are taken. Entry 0 is never
//! handed out: a device whose MSI data was never written names index 0,
//! finds it not remapped, and is blocked and logged. Pure.

/// One bit per entry of a 256-entry table.
pub type Slots = [u64; 4];

/// A fresh table's bitmap, entry 0 already taken.
pub const FRESH: Slots = [1, 0, 0, 0];

/// The lowest free entry, marked taken, or `None` when all are.
pub fn take(slots: &mut Slots) -> Option<u16> {
    for (word, bits) in slots.iter_mut().enumerate() {
        if *bits != u64::MAX {
            let bit = (!*bits).trailing_zeros();
            *bits |= 1 << bit;
            return Some((word * 64) as u16 + bit as u16);
        }
    }
    None
}

/// Hand an entry back; `false` when it was not taken or is entry 0, so a
/// double release never frees an entry another vector holds.
pub fn give(slots: &mut Slots, index: u16) -> bool {
    let (word, bit) = ((index / 64) as usize, index % 64);
    if index == 0 || word >= slots.len() || slots[word] & (1 << bit) == 0 {
        return false;
    }
    slots[word] &= !(1 << bit);
    true
}
