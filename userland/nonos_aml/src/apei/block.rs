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

//! The Generic Error Status Block (ACPI 6.5, 18.3.2.7.1) and its Generic
//! Error Data Entries, each a section typed by GUID. An entry that runs
//! past the block ends the walk there.

use super::le::{u16_at, u32_at};
use alloc::vec::Vec;

const HEADER: usize = 20;
const ENTRY: usize = 64;
/// Revision 3.0 and later add an 8-byte timestamp to the entry header.
const TIMESTAMPED: u16 = 0x0300;
const MAX_ENTRIES: usize = 64;

/// 0 recoverable, 1 fatal, 2 corrected, 3 none.
pub const FATAL: u32 = 1;
pub const CORRECTED: u32 = 2;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub section: [u8; 16],
    pub severity: u32,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    /// Block Status: which kinds of error are valid, and the entry count.
    pub status: u32,
    pub severity: u32,
    pub entries: Vec<Entry>,
}

impl Block {
    /// A block whose status is zero holds nothing new.
    pub fn is_empty(&self) -> bool {
        self.status & 0xF == 0
    }
}

pub fn parse_block(b: &[u8]) -> Option<Block> {
    let head = b.get(..HEADER)?;
    let (status, data_len, severity) =
        (u32_at(head, 0), u32_at(head, 12) as usize, u32_at(head, 16));
    let data = b.get(HEADER..HEADER.checked_add(data_len)?)?;
    let mut entries = Vec::new();
    let mut at = 0usize;
    while entries.len() < MAX_ENTRIES && at + ENTRY <= data.len() {
        let e = &data[at..];
        let revision = u16_at(e, 20);
        let head_len = if revision >= TIMESTAMPED { ENTRY + 8 } else { ENTRY };
        let len = u32_at(e, 24) as usize;
        let Some(body) = e.get(head_len..head_len.checked_add(len)?) else { break };
        let mut section = [0u8; 16];
        section.copy_from_slice(&e[..16]);
        entries.push(Entry { section, severity: u32_at(e, 16), data: body.to_vec() });
        at += head_len + len;
    }
    Some(Block { status, severity, entries })
}
