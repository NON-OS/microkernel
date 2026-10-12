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

//! The modifier keys. A USB boot report carries them as a bitmap in its
//! first octet; the HID host here hands them over as key usages 0xE0 to
//! 0xE7 (HID Usage Tables 1.4, 10), the bit of that bitmap being the
//! usage less 0xE0 (HID 1.11, 8.3). The layout code takes the bitmap.

/// The modifier bit for `usage`, or `None` for a key that is not one.
pub fn modifier_bit(usage: u8) -> Option<u8> {
    (0xe0..=0xe7).contains(&usage).then(|| 1 << (usage - 0xe0))
}

/// Left or right Control, and left Alt: with Space they cycle the layout.
pub fn layout_chord(modifiers: u8, usage: u8) -> bool {
    usage == 0x2c && modifiers & 0x11 != 0 && modifiers & 0x04 != 0
}
