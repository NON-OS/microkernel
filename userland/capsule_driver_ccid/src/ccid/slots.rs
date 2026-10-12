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
//! What the driver remembers of each slot between a client's requests:
//! which it powered, so CLOSE and a client gone power them off, and which
//! the reader said changed. A card swapped between two STATUS requests
//! reads present both times; only the change notice tells the client it
//! now speaks to a different card.

use super::notice::changed;

/// Slots past this are served but not remembered; readers have one or two.
pub const SLOTS: u8 = 64;

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Slots {
    pub powered: u64,
    changed: u64,
}

impl Slots {
    /// A slot-change notice `n` from a reader with slots 0 to `max_slot`.
    pub fn seen(&mut self, n: &[u8], max_slot: u8) {
        for s in 0..=max_slot.min(SLOTS - 1) {
            if changed(n, s) == Some(true) {
                self.changed |= 1 << s;
            }
        }
    }

    /// Whether `slot` changed since the last time asked.
    pub fn take_changed(&mut self, slot: u8) -> bool {
        let bit = bit(slot);
        let was = self.changed & bit != 0;
        self.changed &= !bit;
        was
    }

    pub fn forget_changes(&mut self) {
        self.changed = 0;
    }

    pub fn power(&mut self, slot: u8, on: bool) {
        if on {
            self.powered |= bit(slot);
        } else {
            self.powered &= !bit(slot);
        }
    }

    /// The slots powered, lowest first.
    pub fn each_powered(&self) -> impl Iterator<Item = u8> + '_ {
        (0..SLOTS).filter(move |s| self.powered & 1 << s != 0)
    }
}

fn bit(slot: u8) -> u64 {
    if slot < SLOTS {
        1 << slot
    } else {
        0
    }
}
