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

//! A NONOS key code as the Linux key it came from, which is what a Wayland
//! client reads. Pure, so the host proofs hold it.
//!
//! The input driver resolves a printable key through the active layout and
//! sends the character (shift-a as 'A'), and every other key from its own
//! table (app_skeleton input/keys.rs). A client reads evdev codes through
//! an xkb keymap, so each is taken back to its key on the US layout the
//! shipped keymap describes; shift and caps arrive as modifiers. A
//! character no US key gives is None, and nothing is sent for it.

/// evdev KEY_A to KEY_Z, in the order of the letters.
const LETTERS: [u8; 26] = [
    30, 48, 46, 32, 18, 33, 34, 35, 23, 36, 37, 38, 50, 49, 24, 25, 16, 19, 31, 20, 22, 47, 17, 45,
    21, 44,
];
/// The digit row, '0' to '9', and what shift gives on each.
const DIGITS: [u8; 10] = [11, 2, 3, 4, 5, 6, 7, 8, 9, 10];
const SHIFTED_DIGITS: &[u8; 10] = b")!@#$%^&*(";
/// The other printable keys: plain, shifted, evdev code.
const PUNCT: [(u8, u8, u8); 12] = [
    (b'-', b'_', 12),
    (b'=', b'+', 13),
    (b'[', b'{', 26),
    (b']', b'}', 27),
    (b';', b':', 39),
    (b'\'', b'"', 40),
    (b'`', b'~', 41),
    (b'\\', b'|', 43),
    (b',', b'<', 51),
    (b'.', b'>', 52),
    (b'/', b'?', 53),
    (b' ', b' ', 57),
];

pub fn evdev_of(code: u32) -> Option<u32> {
    let byte = u8::try_from(code).ok().filter(u8::is_ascii);
    let key = match (code, byte) {
        (_, Some(c @ b'a'..=b'z')) => LETTERS[usize::from(c - b'a')],
        (_, Some(c @ b'A'..=b'Z')) => LETTERS[usize::from(c - b'A')],
        (_, Some(c @ b'0'..=b'9')) => DIGITS[usize::from(c - b'0')],
        (_, Some(c)) if SHIFTED_DIGITS.contains(&c) => {
            DIGITS[SHIFTED_DIGITS.iter().position(|s| *s == c)?]
        }
        (_, Some(c)) => PUNCT.iter().find(|p| p.0 == c || p.1 == c).map(|p| p.2).or(match c {
            0x1B => Some(1),
            0x08 => Some(14),
            b'\t' => Some(15),
            0x0D => Some(28),
            _ => None,
        })?,
        _ => return super::evdev_keys::named(code),
    };
    Some(u32::from(key))
}
