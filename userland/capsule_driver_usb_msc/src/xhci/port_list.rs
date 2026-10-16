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

//! The root ports in a port-status reply that have a device on them. Pure,
//! so the host proofs feed it hostile replies.

use alloc::vec::Vec;

use super::wire::DATA_AT;
use crate::protocol::E_IO;

/// A root port that has a device on it, and which class driver holds it.
#[derive(Clone, Copy, Debug)]
pub struct Port {
    pub id: u8,
    pub owner: u8,
}

const PORTSC_CONNECTED: u32 = 1;
/// Each entry: port, owner, two bytes unused, then PORTSC.
const ENTRY: usize = 8;

/// The connected ports in `resp`, a reply of `len` data bytes: a count, three
/// bytes unused, then the entries.
pub fn connected(resp: &[u8], len: usize) -> Result<Vec<Port>, i32> {
    let count = *resp.get(DATA_AT).ok_or(E_IO)? as usize;
    if len < 4 + count * ENTRY {
        return Err(E_IO);
    }
    let entries = resp.get(DATA_AT + 4..DATA_AT + 4 + count * ENTRY).ok_or(E_IO)?;
    Ok(entries
        .chunks_exact(ENTRY)
        .filter(|e| u32::from_le_bytes([e[4], e[5], e[6], e[7]]) & PORTSC_CONNECTED != 0)
        .map(|e| Port { id: e[0], owner: e[1] })
        .collect())
}
