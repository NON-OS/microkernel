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

//! Seeded hostile values for the harnesses.

use crate::constants::RX_DESC_COUNT;

/// Miri runs the same harnesses with fewer rounds; the full counts are for
/// the native fuzz.
pub fn rounds(full: u64) -> u64 {
    if cfg!(miri) {
        300
    } else {
        full
    }
}

pub fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

/// Half the ids name a posted buffer and half are whatever the device
/// wrote, so both the frame path and the drop path run on every seed range.
pub fn hostile_id(s: &mut u64) -> u32 {
    let raw = xorshift(s);
    if raw & 1 == 0 {
        (raw >> 1) as u32 % u32::from(RX_DESC_COUNT)
    } else {
        (raw >> 1) as u32
    }
}

/// A used length: anywhere at all, or near the frame and buffer bounds.
pub fn hostile_len(s: &mut u64) -> u32 {
    let raw = xorshift(s);
    match raw % 3 {
        0 => (raw >> 2) as u32,
        _ => (raw >> 2) as u32 % 2100,
    }
}
