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

//! The IO-APICs a DRHD names (VT-d 3.4, 8.3.1, scope type 3): each one's
//! id and the PCI path to its requester id, which a remapping entry for its
//! lines must name. Pure; kernel_proofs::dmar_scope::ioapic_tests.
//! dmar_ioapic_source turns a path into the requester id.

use super::dmar_scope::MAX_PATH;

pub const SCOPE_IOAPIC: u8 = 3;
pub const MAX_IOAPIC_SCOPES: usize = 8;
const HEADER: usize = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct IoapicScope {
    /// The IO-APIC id, as the MADT gives it.
    pub id: u8,
    pub start_bus: u8,
    pub path: [(u8, u8); MAX_PATH],
    pub path_len: u8,
}

/// Each IO-APIC scope in a scope list from `from`, into the free places of
/// `out`. A malformed, pathless or surplus one is skipped, and its lines get
/// no remapping entry: bound under remapping, they are refused.
pub fn ioapic_scopes(bytes: &[u8], from: usize, out: &mut [Option<IoapicScope>]) {
    let mut at = from;
    while at + 2 <= bytes.len() {
        let len = bytes[at + 1] as usize;
        if len < HEADER || at + len > bytes.len() || !(len - HEADER).is_multiple_of(2) {
            return;
        }
        let hops = (len - HEADER) / 2;
        if bytes[at] == SCOPE_IOAPIC && hops != 0 && hops <= MAX_PATH {
            let mut s = IoapicScope {
                id: bytes[at + 4],
                start_bus: bytes[at + 5],
                path: [(0, 0); MAX_PATH],
                path_len: hops as u8,
            };
            for h in 0..hops {
                s.path[h] = (bytes[at + HEADER + h * 2], bytes[at + HEADER + h * 2 + 1]);
            }
            if let Some(free) = out.iter_mut().find(|o| o.is_none()) {
                *free = Some(s);
            }
        }
        at += len;
    }
}
