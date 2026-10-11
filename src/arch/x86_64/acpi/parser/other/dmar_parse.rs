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

//! Reading a DRHD's flags and device scopes, and any structure's scope list.

use super::dmar_scope::{Scope, UnitScope, FLAG_INCLUDE_PCI_ALL, MAX_PATH, MAX_SCOPES};
use super::dmar_scope::{DRHD_HEADER_LEN, SCOPE_BRIDGE, SCOPE_ENDPOINT, SCOPE_HEADER_LEN};

/// Parse one DRHD structure (`bytes` is exactly its `length` bytes, type 0).
/// `None` for a structure too short to be a DRHD. Scope types other than
/// endpoint and bridge (IOAPIC, HPET, ACPI namespace) name no PCI device and are
/// skipped; a malformed scope ends the walk and marks the unit truncated.
pub fn parse_drhd(bytes: &[u8]) -> Option<UnitScope> {
    if bytes.len() < DRHD_HEADER_LEN || u16::from_le_bytes([bytes[0], bytes[1]]) != 0 {
        return None;
    }
    let mut unit = UnitScope::EMPTY;
    unit.include_all = bytes[4] & FLAG_INCLUDE_PCI_ALL != 0;
    unit.segment = u16::from_le_bytes([bytes[6], bytes[7]]);
    read_scopes(bytes, DRHD_HEADER_LEN, &mut unit);
    Some(unit)
}

/// The device scopes from `from` to the end of a DRHD or RMRR structure,
/// into `unit`. A malformed scope ends the walk and marks it truncated.
pub fn read_scopes(bytes: &[u8], from: usize, unit: &mut UnitScope) {
    let mut at = from;
    while at + 2 <= bytes.len() {
        let kind = bytes[at];
        let len = bytes[at + 1] as usize;
        if len < SCOPE_HEADER_LEN
            || at + len > bytes.len()
            || !(len - SCOPE_HEADER_LEN).is_multiple_of(2)
        {
            unit.truncated = true;
            break;
        }
        if kind == SCOPE_ENDPOINT || kind == SCOPE_BRIDGE {
            let hops = (len - SCOPE_HEADER_LEN) / 2;
            if hops == 0 || hops > MAX_PATH || unit.count as usize >= MAX_SCOPES {
                unit.truncated = true;
            } else {
                let mut s = Scope::EMPTY;
                s.kind = kind;
                s.start_bus = bytes[at + 5];
                for h in 0..hops {
                    let p = at + SCOPE_HEADER_LEN + h * 2;
                    s.path[h] = (bytes[p], bytes[p + 1]);
                }
                s.path_len = hops as u8;
                unit.scopes[unit.count as usize] = s;
                unit.count += 1;
            }
        }
        at += len;
    }
}
