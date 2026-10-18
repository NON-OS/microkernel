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

//! The displays' arrangement and scales as the policy store keeps them, put
//! back on the compositor (server/display_replay.rs). Settings stores a
//! change once the compositor took it; the shell sends the compositor a kept
//! value when the store's value is new to it, and all of them again for a
//! compositor it has not told yet, so the arrangement survives a reboot and
//! a compositor restart. A value is never sent because the screen differs
//! from the store: a live change the store did not take stays as it is.

/// DisplayLayout, then DisplayScale1 to DisplayScale4 (U8 fields).
pub const FIELDS: [u32; 5] = [0x0126, 0x0127, 0x0128, 0x0129, 0x012A];
/// The scales the compositor takes, in quarters; 0 is its own rule.
pub const SCALES: [u8; 7] = [0, 4, 5, 6, 8, 12, 16];

/// Whether `v` is a value field `i` may hold: mirror or extend for the
/// layout, one of the scales for a display.
pub fn valid(i: usize, v: u8) -> bool {
    match i {
        0 => v <= 1,
        1..=4 => SCALES.contains(&v),
        _ => false,
    }
}

/// The compositor body for field `i` holding `v`: OUTPUT_LAYOUT's mode, or
/// OUTPUT_SCALE's display and quarters. None for a value it may not hold.
pub fn body(i: usize, v: u8) -> Option<([u8; 8], usize)> {
    let mut b = [0u8; 8];
    if !valid(i, v) {
        return None;
    }
    if i == 0 {
        b[..4].copy_from_slice(&u32::from(v).to_le_bytes());
        return Some((b, 4));
    }
    b[..4].copy_from_slice(&(i as u32 - 1).to_le_bytes());
    b[4..].copy_from_slice(&u32::from(v).to_le_bytes());
    Some((b, 8))
}
