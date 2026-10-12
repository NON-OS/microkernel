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

//! LPIT (17): the low-power idle states, for power and the shell. The
//! body: a count u8, then each state's id u16, flags u8 (bit 0 disabled,
//! bit 1 has a residency counter, bit 2 the counter was read), minimum
//! residency u32 and wake latency u32 (microseconds), the counter's
//! frequency u64 (0, the TSC's) and its count now u64 (0 when not read).
//! No LPIT is a count of 0.

use crate::suspend::lpit::states;
use alloc::vec::Vec;

pub fn lpit_wire() -> (i32, Vec<u8>) {
    let all = states();
    let mut b = alloc::vec![all.len() as u8];
    for s in &all {
        b.extend_from_slice(&s.id.to_le_bytes());
        b.push(s.flags);
        b.extend_from_slice(&s.min_residency_us.to_le_bytes());
        b.extend_from_slice(&s.latency_us.to_le_bytes());
        b.extend_from_slice(&s.frequency.to_le_bytes());
        b.extend_from_slice(&s.count.unwrap_or(0).to_le_bytes());
    }
    (0, b)
}
