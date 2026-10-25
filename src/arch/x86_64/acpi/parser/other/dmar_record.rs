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

//! Keeping one DRHD the DMAR parse reads: its register base and what it
//! covers, filled in place in dmar_units' statics, and the boot line when its
//! scope list was cut short.

use super::super::dmar_keep::{keep, Unit};
use super::super::dmar_namespace_store::record_namespace_scopes;
use super::super::dmar_scope::{parse_drhd, UnitScope, DRHD_HEADER_LEN, MAX_SCOPES};
use super::super::dmar_scope_count::pci_scopes_named;
use super::super::dmar_source_store::record_source_scopes;
use super::super::unit_base::DRHD_WINDOW;
use super::{REMAP_UNIT_BASES, REMAP_UNIT_SCOPES};

/// Keep one DRHD (`bytes` exactly its length); true when it was kept. One
/// whose scopes do not parse is kept truncated, never taken to cover a device.
pub(in super::super) fn record_drhd(bytes: &[u8], segment: u16, base: u64) -> bool {
    let mut bases = REMAP_UNIT_BASES.lock();
    let mut scopes = REMAP_UNIT_SCOPES.lock();
    let fits = keep(segment, bases.len().max(scopes.len())) == Unit::Kept;
    if !fits || super::super::unit_base_env::admit(base, DRHD_WINDOW, &bases).is_err() {
        drop((bases, scopes));
        super::left_off();
        return false;
    }
    record_namespace_scopes(bases.len() as u8, bytes);
    record_source_scopes(bases.len() as u8, bytes);
    super::super::dmar_ioapic_table::record(bytes);
    if bases.push(base).is_err() || scopes.push(UnitScope::EMPTY).is_err() {
        return false;
    }
    // Filled in place, in the static: never a unit's copy on the stack.
    let n = scopes.len() - 1;
    let unit = &mut scopes[n];
    if !parse_drhd(bytes, unit) {
        unit.truncated = true;
    }
    if unit.truncated {
        let named = pci_scopes_named(bytes, DRHD_HEADER_LEN);
        let mut line = [0u8; super::truncated::LINE_MAX];
        let len = super::truncated::line(n, named, MAX_SCOPES, &mut line);
        crate::sys::serial::println(&line[..len]);
    }
    true
}
