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

//! What a report becomes. Keys are usages of the HID Keyboard/Keypad page
//! (HID Usage Tables 1.4, 10), the modifiers among them as 0xE0 to 0xE7, so
//! the capsule maps them with the same keymap the USB keyboard driver uses
//! and posts them to the input ring.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Input {
    KeyDown(u8),
    KeyUp(u8),
    /// Mouse button 1 to 3 (HID 1.11, B.2).
    ButtonDown(u8),
    ButtonUp(u8),
    /// Relative motion; positive x is right, positive y is down.
    Motion {
        dx: i16,
        dy: i16,
    },
}
