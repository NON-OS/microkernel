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

//! The Processor Hierarchy Node (type 0: flags, parent, ACPI processor
//! ID, then its private resources as table offsets; 20 bytes and four per
//! resource).

use super::super::le::u32_at;
use alloc::vec::Vec;

pub(crate) const NODE_MIN: usize = 20;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProcessorNode {
    /// Where it sits in the table: what parents and resources name.
    pub offset: u32,
    /// Bit 0 a physical package, 1 the ID is valid, 2 a thread, 3 a leaf,
    /// 4 every child identical.
    pub flags: u32,
    pub parent: Option<u32>,
    pub acpi_id: Option<u32>,
    pub resources: Vec<u32>,
}

/// None when the structure cannot hold the resources it counts.
pub(crate) fn node(s: &[u8], offset: u32) -> Option<ProcessorNode> {
    let n = u32_at(s, 16)? as usize;
    if n > (s.len() - NODE_MIN) / 4 {
        return None;
    }
    let flags = u32_at(s, 4)?;
    let resources = (0..n).map(|i| u32_at(s, NODE_MIN + 4 * i)).collect::<Option<Vec<_>>>()?;
    Some(ProcessorNode {
        offset,
        flags,
        parent: u32_at(s, 8).filter(|p| *p != 0),
        acpi_id: u32_at(s, 12).filter(|_| flags & 2 != 0),
        resources,
    })
}
