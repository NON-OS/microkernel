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

//! LPIT's low-power idle states from the table's bytes, with no pointer
//! read: `lpit_load` keeps what it yields, `kernel_proofs` mounts these
//! files by path. Native C-states (type 0) are read by `lpit_decode`;
//! other types are skipped; a short or overrunning structure ends the walk
//! with what came before.

use super::lpit_decode::{native, u32_at, NATIVE_LEN};
use alloc::vec::Vec;

/// SDT header; the structures follow it directly.
pub const STATES_AT: usize = 36;
/// States kept; a laptop lists two or three.
pub const MAX_STATES: usize = 16;

/// A residency counter: its register, as the GAS gives it, and its rate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Counter {
    pub space: u8,
    pub bit_width: u8,
    pub bit_offset: u8,
    pub access_size: u8,
    pub address: u64,
    /// Counts per second; 0 when the counter runs at the TSC's rate.
    pub frequency: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LpiState {
    pub id: u16,
    pub disabled: bool,
    pub min_residency_us: u32,
    pub latency_us: u32,
    pub counter: Option<Counter>,
}

pub fn states(table: &[u8]) -> Vec<LpiState> {
    let (mut out, mut at) = (Vec::new(), STATES_AT);
    while let Some(h) = table.get(at..at + 8) {
        let len = u32_at(h, 4) as usize;
        let Some(s) = table.get(at..at.saturating_add(len)).filter(|_| len >= 8) else { break };
        if u32_at(h, 0) == 0 && len >= NATIVE_LEN && out.len() < MAX_STATES {
            out.push(native(s));
        }
        at += len;
    }
    out
}
