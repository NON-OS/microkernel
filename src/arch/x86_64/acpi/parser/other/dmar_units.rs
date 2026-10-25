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

use super::dmar_keep::{keep, Unit, MAX_REMAP_UNITS};
use super::dmar_namespace_store::record_namespace_scopes;
use super::dmar_scope::{parse_drhd, UnitScope};
use super::dmar_source_store::record_source_scopes;
use super::unit_base::DRHD_WINDOW;

static REMAP_UNIT_BASES: Mutex<heapless::Vec<u64, MAX_REMAP_UNITS>> =
    Mutex::new(heapless::Vec::new());
static LEFT_OFF: Mutex<usize> = Mutex::new(0);
/// What each unit in `REMAP_UNIT_BASES` covers, at the same index.
static REMAP_UNIT_SCOPES: Mutex<heapless::Vec<UnitScope, MAX_REMAP_UNITS>> =
    Mutex::new(heapless::Vec::new());

/// Register bases of the segment 0 remapping units DMAR reported, in table
/// order: often two, one for the integrated graphics and one for the rest.
/// All are programmed; a device is translated by the unit whose scope holds it.
pub fn remap_unit_bases() -> heapless::Vec<u64, MAX_REMAP_UNITS> {
    REMAP_UNIT_BASES.lock().clone()
}

/// Units DMAR listed that the kernel left off: on a segment it does not
/// enumerate, past the eight it holds, with a register base it refused, or
/// past a structure it could not read. Their devices reach memory untranslated.
pub fn units_left_off() -> usize {
    *LEFT_OFF.lock()
}

pub(super) fn left_off() {
    *LEFT_OFF.lock() += 1;
}

/// What each segment 0 remapping unit covers, in DMAR order (index-matched to
/// `remap_unit_bases`).
pub fn remap_unit_scopes() -> heapless::Vec<UnitScope, MAX_REMAP_UNITS> {
    REMAP_UNIT_SCOPES.lock().clone()
}

/// Keep one DRHD (`bytes` exactly its length); true when it was kept. One
/// whose scopes do not parse is kept truncated, never taken to cover a device.
pub(super) fn record_drhd(bytes: &[u8], segment: u16, base: u64) -> bool {
    let mut bases = REMAP_UNIT_BASES.lock();
    let mut scopes = REMAP_UNIT_SCOPES.lock();
    let fits = keep(segment, bases.len().max(scopes.len())) == Unit::Kept;
    if !fits || super::unit_base_env::admit(base, DRHD_WINDOW, &bases).is_err() {
        drop((bases, scopes));
        left_off();
        return false;
    }
    record_namespace_scopes(bases.len() as u8, bytes);
    record_source_scopes(bases.len() as u8, bytes);
    super::dmar_ioapic_table::record(bytes);
    let scope = parse_drhd(bytes).unwrap_or(UnitScope { truncated: true, ..UnitScope::EMPTY });
    bases.push(base).is_ok() && scopes.push(scope).is_ok()
}
