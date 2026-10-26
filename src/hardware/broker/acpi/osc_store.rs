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

//! The root bridges' _OSC grants as the ACPI capsule published them, kept
//! for the kernel to read: `osc_grant` is its only reader, and nothing
//! outside the kernel sees it. A bridge published again is replaced.

use super::osc_grant::{grant_for, OscGrant};
use crate::sys::serial::Line;
use alloc::vec::Vec;
use spin::Mutex;

const MAX_BRIDGES: usize = 64;

static GRANTS: Mutex<Vec<OscGrant>> = Mutex::new(Vec::new());

/// False when the store is full.
pub fn publish(g: OscGrant) -> bool {
    let mut grants = GRANTS.lock();
    grants.retain(|x| (x.segment, x.first) != (g.segment, g.first));
    if grants.len() >= MAX_BRIDGES {
        return false;
    }
    grants.push(g);
    Line::new()
        .str(b"[ACPI] _OSC segment=")
        .hex(u64::from(g.segment))
        .str(b" buses=")
        .hex(u64::from(g.first))
        .str(b"-")
        .hex(u64::from(g.last))
        .str(b" offered=")
        .hex(u64::from(g.offered))
        .str(b" granted=")
        .hex(u64::from(g.granted))
        .end();
    true
}

/// What firmware left to NONOS for the function at `segment:bus`; None
/// when no root bridge holding that bus was published, which means
/// firmware keeps every control.
pub fn osc_grant(segment: u16, bus: u8) -> Option<OscGrant> {
    grant_for(&GRANTS.lock(), segment, bus)
}
