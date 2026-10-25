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

//! The device scope list that ends a DRHD or an RMRR structure: each entry
//! names a PCI endpoint or bridge by the bus its path starts on and the
//! (device, function) hops below it. Pure, so the host proofs hold it.

use super::dmar_scope::{Scope, MAX_PATH, SCOPE_BRIDGE, SCOPE_ENDPOINT};

pub const SCOPE_HEADER_LEN: usize = 6;

/// Walk the scopes in `bytes[at..]` into `out`. Returns how many were kept and
/// whether any PCI scope was dropped (too many, a path too long or empty, or a
/// malformed entry, which also ends the walk). Scope types that name no PCI
/// device (IOAPIC, HPET, ACPI namespace) are skipped.
pub fn walk_scopes(bytes: &[u8], mut at: usize, out: &mut [Scope]) -> (u8, bool) {
    let (mut count, mut truncated) = (0usize, false);
    while at + 2 <= bytes.len() {
        let kind = bytes[at];
        let len = bytes[at + 1] as usize;
        if len < SCOPE_HEADER_LEN
            || at + len > bytes.len()
            || !(len - SCOPE_HEADER_LEN).is_multiple_of(2)
        {
            truncated = true;
            break;
        }
        if kind == SCOPE_ENDPOINT || kind == SCOPE_BRIDGE {
            let hops = (len - SCOPE_HEADER_LEN) / 2;
            if hops == 0 || hops > MAX_PATH || count >= out.len().min(u8::MAX as usize) {
                truncated = true;
            } else {
                let mut path = [(0u8, 0u8); MAX_PATH];
                for (h, hop) in path.iter_mut().enumerate().take(hops) {
                    let p = at + SCOPE_HEADER_LEN + h * 2;
                    *hop = (bytes[p], bytes[p + 1]);
                }
                out[count] = Scope { kind, start_bus: bytes[at + 5], path, path_len: hops as u8 };
                count += 1;
            }
        }
        at += len;
    }
    (count as u8, truncated)
}

/// Whether a scope in `bytes[at..]` is of a kind other than a PCI endpoint or
/// bridge. A malformed entry ends the walk; `walk_scopes` marks it truncated.
pub fn names_off_pci(bytes: &[u8], mut at: usize) -> bool {
    while let (Some(&kind), Some(&len)) = (bytes.get(at), bytes.get(at + 1)) {
        let len = len as usize;
        if len < SCOPE_HEADER_LEN || at + len > bytes.len() {
            return false;
        }
        if kind != SCOPE_ENDPOINT && kind != SCOPE_BRIDGE {
            return true;
        }
        at += len;
    }
    false
}
