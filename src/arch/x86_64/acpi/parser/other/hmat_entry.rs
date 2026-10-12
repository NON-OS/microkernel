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

//! HMAT's structures from the table's bytes, with no pointer read: the one
//! HMAT decoder (`hmat_decode` reads each structure), beside SRAT's. The
//! x86 parser keeps what it yields; the aarch64 tables and `kernel_proofs`
//! mount these files by path. Layouts are recalled from ACPI 6.5, 5.2.28,
//! not read: unverified. Memory side cache structures are skipped; a short
//! or overrunning structure ends the walk with what came before.

use super::hmat_decode::{attributes, matrix};
use alloc::vec::Vec;

/// SDT header, four reserved bytes.
pub const STRUCTURES_AT: usize = 40;
/// Structures kept; a large server lists a few dozen.
pub const MAX_STRUCTURES: usize = 256;

/// Memory Proximity Domain Attributes: the initiator a memory domain is
/// attached to, when firmware says one is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Attributes {
    pub memory: u32,
    pub initiator: Option<u32>,
}

/// One latency (`data_type` 0 to 2, picoseconds) or bandwidth (3 to 5,
/// MB/s) matrix at one memory hierarchy level (0 memory, 1 to 3 caches).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matrix {
    pub hierarchy: u8,
    pub data_type: u8,
    pub base_unit: u64,
    pub initiators: Vec<u32>,
    pub targets: Vec<u32>,
    pub entries: Vec<u16>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Hmat {
    pub attributes: Vec<Attributes>,
    pub matrices: Vec<Matrix>,
}

pub fn structures(table: &[u8]) -> Hmat {
    let (mut out, mut at, mut n) = (Hmat::default(), STRUCTURES_AT, 0);
    while let Some(h) = table.get(at..at + 8) {
        let len = u32::from_le_bytes([h[4], h[5], h[6], h[7]]) as usize;
        let Some(s) = table.get(at..at.saturating_add(len)).filter(|_| len >= 8) else { break };
        match u16::from_le_bytes([h[0], h[1]]) {
            0 => out.attributes.extend(attributes(s)),
            1 => out.matrices.extend(matrix(s)),
            _ => {}
        }
        (at, n) = (at + len, n + 1);
        if n >= MAX_STRUCTURES {
            break;
        }
    }
    out
}
