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

//! The firmware tables the ACPI capsule loaded, measured as the kernel
//! handed them (F-90), pure. Each is one 44-byte record: its signature, its
//! instance (u32), its length (u32) and the BLAKE3 digest of its bytes,
//! big-endian like the attest registry. The list is in hand-out order, the
//! capsule's load order, with an identical repeat skipped and a changed
//! one kept as a new record. One digest folds the domain, the count and
//! every record, so a verifier can tell when a machine's AML changed.

pub const DOMAIN: &[u8] = b"nonos.acpi.tables.v1";
pub const RECORD_LEN: usize = 44;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Measured {
    pub sig: [u8; 4],
    pub instance: u32,
    pub len: u32,
    pub digest: [u8; 32],
}

impl Measured {
    pub fn of(sig: [u8; 4], instance: u32, bytes: &[u8]) -> Measured {
        let digest = *blake3::hash(bytes).as_bytes();
        Measured { sig, instance, len: bytes.len() as u32, digest }
    }

    pub fn encode(&self) -> [u8; RECORD_LEN] {
        let mut out = [0u8; RECORD_LEN];
        out[..4].copy_from_slice(&self.sig);
        out[4..8].copy_from_slice(&self.instance.to_be_bytes());
        out[8..12].copy_from_slice(&self.len.to_be_bytes());
        out[12..].copy_from_slice(&self.digest);
        out
    }
}

/// Adds `m` unless the same table with the same bytes is already listed;
/// true when the list changed.
pub fn note(list: &mut alloc::vec::Vec<Measured>, m: Measured) -> bool {
    let seen = list.contains(&m);
    if !seen {
        list.push(m);
    }
    !seen
}

pub fn digest(list: &[Measured]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(DOMAIN);
    h.update(&(list.len() as u32).to_be_bytes());
    for m in list {
        h.update(&m.encode());
    }
    *h.finalize().as_bytes()
}
