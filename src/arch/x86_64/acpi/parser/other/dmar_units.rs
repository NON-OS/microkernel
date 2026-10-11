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

//! The remapping units (DRHD) DMAR listed: where their registers are and
//! which devices each covers.

use spin::Mutex;

use super::dmar_scope::{parse_drhd, UnitScope};

const MAX_REMAP_UNITS: usize = 8;

static REMAP_UNIT_BASES: Mutex<heapless::Vec<u64, MAX_REMAP_UNITS>> =
    Mutex::new(heapless::Vec::new());
static FOREIGN_SEGMENT_UNITS: Mutex<usize> = Mutex::new(0);
/// What each unit in `REMAP_UNIT_BASES` covers, at the same index.
static REMAP_UNIT_SCOPES: Mutex<heapless::Vec<UnitScope, MAX_REMAP_UNITS>> =
    Mutex::new(heapless::Vec::new());

/// Register bases of the segment 0 remapping units DMAR reported, in table
/// order. A client Intel machine has two: one scoped to the integrated
/// graphics and one for every other device. The VT-d driver programs all of
/// them; a device's DMA is translated only by the unit whose scope holds it.
pub fn remap_unit_bases() -> heapless::Vec<u64, MAX_REMAP_UNITS> {
    REMAP_UNIT_BASES.lock().clone()
}

/// Units DMAR placed on a PCI segment other than 0. The kernel enumerates
/// segment 0 only, so it has no table for their devices and leaves them off.
pub fn foreign_segment_units() -> usize {
    *FOREIGN_SEGMENT_UNITS.lock()
}

/// What each segment 0 remapping unit covers, in DMAR order (index-matched to
/// `remap_unit_bases`).
pub fn remap_unit_scopes() -> heapless::Vec<UnitScope, MAX_REMAP_UNITS> {
    REMAP_UNIT_SCOPES.lock().clone()
}

/// Keep one DRHD (`bytes` exactly its length). True when a segment 0 unit
/// was kept. A unit whose structure does not parse is kept with an empty,
/// truncated scope, so it is never taken to cover a device.
pub(super) fn record(bytes: &[u8], segment: u16, base: u64) -> bool {
    if segment != 0 {
        *FOREIGN_SEGMENT_UNITS.lock() += 1;
        return false;
    }
    let scope = parse_drhd(bytes).unwrap_or(UnitScope { truncated: true, ..UnitScope::EMPTY });
    let mut bases = REMAP_UNIT_BASES.lock();
    let mut scopes = REMAP_UNIT_SCOPES.lock();
    if bases.len() >= MAX_REMAP_UNITS || scopes.len() >= MAX_REMAP_UNITS {
        return false;
    }
    bases.push(base).is_ok() && scopes.push(scope).is_ok()
}
