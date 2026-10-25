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

//! Reading a DRHD's flags and device scopes, on the one scope walk an RMRR
//! shares (dmar_scope_walk.rs).

use super::dmar_scope::{UnitScope, DRHD_HEADER_LEN, FLAG_INCLUDE_PCI_ALL};
use super::dmar_scope_walk::walk_scopes;

/// Parse one DRHD structure (`bytes` is exactly its `length` bytes, type 0)
/// into `unit`, an empty one, in place: at 255 scopes a unit is too large to
/// pass by value. False for a structure too short to be a DRHD. Scope types
/// other than endpoint and bridge (IOAPIC, HPET, ACPI namespace) name no PCI
/// device and are skipped; a malformed scope ends the walk and marks the unit
/// truncated.
pub fn parse_drhd(bytes: &[u8], unit: &mut UnitScope) -> bool {
    if bytes.len() < DRHD_HEADER_LEN || u16::from_le_bytes([bytes[0], bytes[1]]) != 0 {
        return false;
    }
    unit.include_all = bytes[4] & FLAG_INCLUDE_PCI_ALL != 0;
    unit.segment = u16::from_le_bytes([bytes[6], bytes[7]]);
    (unit.count, unit.truncated) = walk_scopes(bytes, DRHD_HEADER_LEN, &mut unit.scopes);
    true
}
