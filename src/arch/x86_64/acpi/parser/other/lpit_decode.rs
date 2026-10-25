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

//! One native C-state structure (56 bytes): type u32, length u32, unique
//! id u16, two reserved bytes, flags u32 (bit 0 disabled, bit 1 no
//! residency counter), the entry trigger GAS at 16, the minimum residency
//! u32 at 28 and the wake latency u32 at 32 (both microseconds), the
//! residency counter GAS at 36 and its frequency u64 at 48 (0: the TSC's).
//! Recalled from Intel's published LPIT description, not read:
//! unverified. The entry trigger is not kept: NONOS's idle enters through
//! the processor's own C-states, not this register.

use super::lpit_entry::{Counter, LpiState};

pub(super) const NATIVE_LEN: usize = 56;

pub(super) fn u32_at(s: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([s[at], s[at + 1], s[at + 2], s[at + 3]])
}

fn u64_at(s: &[u8], at: usize) -> u64 {
    u64::from(u32_at(s, at)) | u64::from(u32_at(s, at + 4)) << 32
}

/// `s` is at least `NATIVE_LEN` long.
pub(super) fn native(s: &[u8]) -> LpiState {
    let flags = u32_at(s, 12);
    let address = u64_at(s, 40);
    let counter = (flags & 2 == 0 && address != 0).then(|| Counter {
        space: s[36],
        bit_width: s[37],
        bit_offset: s[38],
        access_size: s[39],
        address,
        frequency: u64_at(s, 48),
    });
    LpiState {
        id: u16::from_le_bytes([s[8], s[9]]),
        disabled: flags & 1 != 0,
        min_residency_us: u32_at(s, 28),
        latency_us: u32_at(s, 32),
        counter,
    }
}
