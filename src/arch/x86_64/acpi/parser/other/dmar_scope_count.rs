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

//! How many PCI device scopes a DRHD or RMRR names, kept or not: what the
//! boot line for a truncated unit counts (dmar_truncated.rs). Pure, so the
//! host proofs hold it.

use super::dmar_scope::{SCOPE_BRIDGE, SCOPE_ENDPOINT};
use super::dmar_scope_walk::SCOPE_HEADER_LEN;

/// The endpoint and bridge scopes in `bytes[at..]`, up to the first malformed
/// entry, which ends the count as it ends `walk_scopes`.
pub(crate) fn pci_scopes_named(bytes: &[u8], mut at: usize) -> u16 {
    let mut named = 0u16;
    while let (Some(&kind), Some(&len)) = (bytes.get(at), bytes.get(at + 1)) {
        let len = len as usize;
        if len < SCOPE_HEADER_LEN || at + len > bytes.len() {
            break;
        }
        if kind == SCOPE_ENDPOINT || kind == SCOPE_BRIDGE {
            named = named.saturating_add(1);
        }
        at += len;
    }
    named
}
