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

//! LPIT's low-power idle states as the kernel reads them (MkAcpiLpit),
//! read-only: for suspend-to-idle to say how far each state's residency
//! counter advanced, and for LPIT (17, `server/lpit_wire.rs`).

use crate::sys::log::Line;
use crate::sys::lpit::{lpit, COUNTER, FREQUENCY, ID_FLAGS, LATENCY, MIN_RESIDENCY};
use alloc::vec::Vec;

/// As many as the kernel keeps.
const MAX_STATES: u32 = 16;

/// Flags: bit 0 disabled, bit 1 has a residency counter, bit 2 the
/// counter was read.
pub struct State {
    pub id: u16,
    pub flags: u8,
    pub min_residency_us: u32,
    pub latency_us: u32,
    pub frequency: u64,
    pub count: Option<u64>,
}

pub fn states() -> Vec<State> {
    let mut out = Vec::new();
    for i in 0..MAX_STATES {
        let Ok(w) = lpit(i, ID_FLAGS) else { break };
        let field = |f| lpit(i, f).unwrap_or(0);
        let count = (w >> 17 & 1 != 0).then(|| lpit(i, COUNTER).ok()).flatten();
        let flags = (w >> 16) as u8 & 3 | u8::from(count.is_some()) << 2;
        let (min_residency_us, latency_us) = (field(MIN_RESIDENCY) as u32, field(LATENCY) as u32);
        let frequency = if flags & 2 != 0 { field(FREQUENCY) } else { 0 };
        out.push(State { id: w as u16, flags, min_residency_us, latency_us, frequency, count });
    }
    out
}

/// One line per counted state: the counts it advanced between `before`
/// and `after`, read around a suspend-to-idle.
pub fn note_residency(before: &[State], after: &[State]) {
    for (a, b) in before.iter().zip(after) {
        if let (Some(x), Some(y)) = (a.count, b.count) {
            let l = Line::new(b"s2idle: LPI state ").dec(u64::from(a.id));
            l.str(b" counter advanced ").dec(y.wrapping_sub(x)).end();
        }
    }
}
