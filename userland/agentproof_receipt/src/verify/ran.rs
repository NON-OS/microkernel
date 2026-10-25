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

//! What else the machine ran, from the registry's entries (pid, measurement,
//! capabilities, authority, as src/security/attest_registry/format.rs lays
//! them out). They are believed only once they fold to the signed root.

use alloc::vec::Vec;

use nsh_attest::steps::NETWORK;

use crate::napr::ENTRY;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ran {
    pub pid: u32,
    pub measurement: [u8; 32],
    pub caps: u64,
    /// 0 the vendor's root, 1 + slot a developer's, 255 a publisher's.
    pub authority: u8,
}

impl Ran {
    pub fn network(&self) -> bool {
        self.caps & NETWORK != 0
    }
}

pub fn ran(entries: &[u8]) -> Vec<Ran> {
    entries
        .chunks_exact(ENTRY)
        .map(|e| Ran {
            pid: u32::from_be_bytes([e[0], e[1], e[2], e[3]]),
            measurement: e[4..36].try_into().unwrap_or([0; 32]),
            caps: u64::from_be_bytes(e[36..44].try_into().unwrap_or([0; 8])),
            authority: e[44],
        })
        .collect()
}
