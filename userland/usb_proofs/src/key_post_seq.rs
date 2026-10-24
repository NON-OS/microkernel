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

//! The seeded key events the key-post tests read, and the driver's flags as
//! they were before a caller could add to them.

use crate::hid::key_event::KeyEvent;

/// The driver's flags before callers could add to them, restated.
pub fn flags_before(modifiers: u8, caps: bool) -> u16 {
    let bit = |mask: u8, at: u16| u16::from(modifiers & mask != 0) << at;
    bit(0x22, 0) | bit(0x11, 1) | bit(0x04, 2) | bit(0x88, 3) | u16::from(caps) << 4 | bit(0x40, 6)
}

pub fn xorshift(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

/// Seeded key events: scancodes from a small set so presses and releases
/// pair up, any modifiers, Caps Lock either way, and the code each resolved
/// to.
pub fn events(seed: u64) -> Vec<(KeyEvent, u32)> {
    let mut s = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    (0..64)
        .map(|_| {
            let r = xorshift(&mut s);
            let scancode = [0x04, 0x05, 0x16, 0x28, 0x29, 0x2C, 0x35, 0x64][r as usize % 8];
            let (modifiers, pressed, caps) = ((r >> 8) as u8, r >> 16 & 1 == 1, r >> 17 & 1 == 1);
            let ev = KeyEvent { scancode, ascii: 0, modifiers, pressed, caps };
            (ev, [0x61, 0x41, 0x2028, 0x0D][(r >> 20) as usize % 4])
        })
        .collect()
}
