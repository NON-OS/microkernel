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

//! The walk over a DRHD's device scopes that keeps the numbered ones (ACPI
//! namespace devices, I/O APICs, HPETs) of one type, beside the PCI walk in
//! `dmar_scope_walk`. Pure.

use super::dmar_namespace_scope::{NamespaceScope, SCOPE_NAMESPACE};
use super::dmar_scope::MAX_PATH;
use super::dmar_scope_walk::SCOPE_HEADER_LEN;

/// Walk the scopes in `bytes[at..]`, keeping the namespace ones in `out`.
/// Returns how many were kept and whether any was dropped (too many, an
/// empty or long path, or a malformed entry, which also ends the walk).
pub fn walk_namespace_scopes(bytes: &[u8], at: usize, out: &mut [NamespaceScope]) -> (u8, bool) {
    walk_numbered_scopes(bytes, at, SCOPE_NAMESPACE, out)
}

/// As `walk_namespace_scopes`, for the scopes of type `kind`.
pub fn walk_numbered_scopes(
    bytes: &[u8],
    mut at: usize,
    kind: u8,
    out: &mut [NamespaceScope],
) -> (u8, bool) {
    let (mut count, mut dropped) = (0usize, false);
    while at + 2 <= bytes.len() {
        let len = bytes[at + 1] as usize;
        if len < SCOPE_HEADER_LEN
            || at + len > bytes.len()
            || !(len - SCOPE_HEADER_LEN).is_multiple_of(2)
        {
            dropped = true;
            break;
        }
        if bytes[at] == kind {
            let hops = (len - SCOPE_HEADER_LEN) / 2;
            if hops == 0 || hops > MAX_PATH || count >= out.len() {
                dropped = true;
            } else {
                let mut path = [(0u8, 0u8); MAX_PATH];
                for (h, hop) in path.iter_mut().enumerate().take(hops) {
                    let p = at + SCOPE_HEADER_LEN + h * 2;
                    *hop = (bytes[p], bytes[p + 1]);
                }
                let (number, start_bus) = (bytes[at + 4], bytes[at + 5]);
                out[count] = NamespaceScope { number, start_bus, path, path_len: hops as u8 };
                count += 1;
            }
        }
        at += len;
    }
    (count as u8, dropped)
}
