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

//! The keystrokes.
//!
//! There is no ceiling here beyond the token's own precision. What can actually
//! be spent is decided by the balance at signing time; silently rewriting a
//! figure while it is being typed would hide that refusal rather than prevent it.

use super::Amount;

/// Past thirty figures the amount exceeds any supply that exists, and further
/// digits would saturate rather than carry.
const MAX_DIGITS: u32 = 30;

impl Amount {
    /// Type one digit. `decimals` is the token's precision, which is where the
    /// fraction stops: a place the chain cannot count would silently multiply the
    /// amount rather than extend it.
    pub fn digit(&mut self, d: u8, decimals: u32) {
        if d > 9 {
            return;
        }
        // A figure that arrived from a bar or a max shortcut is a whole amount,
        // not a prefix. Typing over it starts again rather than appending a digit
        // to somebody else's number, which would multiply a balance by ten.
        if self.typed == 0 && self.value != 0 {
            self.clear();
        }
        if self.typed >= MAX_DIGITS || (self.point && self.places >= decimals) {
            return;
        }
        self.value = self.value.saturating_mul(10).saturating_add(d as u128);
        self.typed += 1;
        if self.point {
            self.places += 1;
        }
    }

    /// Start the fraction. A second point is ignored rather than refused, because
    /// a reader who types one twice meant it once.
    pub fn start_point(&mut self) {
        self.point = true;
    }

    /// Take back the last keystroke. Undoing the digit after a point removes the
    /// digit; undoing again removes the point, so backspace retraces exactly the
    /// keys that were pressed.
    pub fn backspace(&mut self) {
        if self.places > 0 {
            self.places -= 1;
        } else if self.point {
            self.point = false;
            return;
        }
        self.value /= 10;
        self.typed = self.typed.saturating_sub(1);
    }
}
