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

/// A usage the HID keyboard page gives a key: 0x04 to 0xA4, and the keypad
/// block 0xB0 to 0xDD. 0x00 is an empty slot and 0x01 to 0x03 are error
/// codes. 0xE0 to 0xE7 are the modifiers, which a boot report carries as
/// bits in its first byte, not in a key slot. Every other value is reserved
/// and makes no key event.
pub(in crate::hid::keyboard) fn is_real_key(key: u8) -> bool {
    matches!(key, 0x04..=0xA4 | 0xB0..=0xDD)
}
