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

//! Step 7: the registry. The entries fold to the root the quote bound,
//! the machine says it recorded everything it runs, the count it states is
//! the count it sent, and exactly one capsule holds Network: the release's
//! `nsh.front`, admitted by the vendor's root.

use super::format::{DOMAIN, ENTRY_LEN};
use crate::{Doc, Refusal, Release};

/// `Capability::Network` in the entry's mask (src/capabilities/types).
pub const NETWORK: u64 = 1 << 2;
/// The authority byte of a capsule the vendor's root admitted.
pub const VENDOR: u8 = 0;

fn network(e: &[u8]) -> bool {
    let caps = u64::from_be_bytes([e[36], e[37], e[38], e[39], e[40], e[41], e[42], e[43]]);
    caps & NETWORK != 0
}

/// The root the kernel folds these entries to: the domain, the count from
/// the byte length, then the entries.
pub fn fold(entries: &[u8]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(DOMAIN);
    h.update(&((entries.len() / ENTRY_LEN) as u32).to_be_bytes());
    h.update(entries);
    *h.finalize().as_bytes()
}

pub fn registry(doc: &Doc<'_>, entries: &[u8], release: &Release) -> Result<(), Refusal> {
    // Whole records, 1 to 1024: `Evidence::verify` read them with nsh_wire.
    let n = entries.len() / ENTRY_LEN;
    if fold(entries) != *doc.registry_root {
        return Err(Refusal::Registry);
    }
    if !doc.registry_complete {
        return Err(Refusal::RegistryIncomplete);
    }
    if doc.capsule_count as usize != n {
        return Err(Refusal::CapsuleCount);
    }
    let net = entries.chunks_exact(ENTRY_LEN).filter(|e| network(e));
    let fronts = net
        .map(|e| e[4..36] == release.front && e[44] == VENDOR)
        .try_fold(0, |n, ok| ok.then_some(n + 1));
    match fronts {
        Some(1) => Ok(()),
        _ => Err(Refusal::Front),
    }
}
