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

//! Setting the figure from something other than the keypad.

use super::Amount;

impl Amount {
    pub fn clear(&mut self) {
        *self = Amount::new();
    }

    /// Replace the figure with one that did not come from the keypad, such as a
    /// drag on a bar or a max shortcut.
    pub fn set_scaled(&mut self, scaled: u128, decimals: u32) {
        self.value = scaled;
        self.places = decimals;
        self.typed = 0;
        self.point = decimals > 0;
    }
}
