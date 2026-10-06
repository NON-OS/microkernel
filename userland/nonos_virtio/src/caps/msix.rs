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

//! Where the MSI-X table and pending-bit array sit, so no register structure
//! is taken from pages the broker will not map.
//!
//! The kernel programs the table itself and never maps it, or the PBA, into
//! a capsule: a mapping that reaches either is cut short at the page below
//! it, and one that starts inside is refused. On QEMU the table has a BAR to
//! itself (BAR1 of a modern virtio function), which is exactly the BAR a
//! legacy-minded driver used to pick. A capability that names those pages is
//! one this driver cannot use, so the parse skips it like any other.

use super::layout::{MSIX_CAP_LEN, MSIX_CONTROL, MSIX_PBA, MSIX_TABLE};
use crate::pci::{ConfigSpace, CONFIG_SPACE_LEN};

const ENTRY_BYTES: u64 = 16;
const TABLE_SIZE_MASK: u16 = 0x7FF;
const BIR_MASK: u32 = 0x7;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MsixLayout {
    pub table_bar: u8,
    /// Byte range `[start, end)` inside `table_bar`.
    pub table: (u64, u64),
    pub pba_bar: u8,
    pub pba: (u64, u64),
}

impl MsixLayout {
    /// The layout an MSI-X capability at `ptr` describes, sized the way the
    /// kernel sizes it when it decides what to refuse.
    pub fn read(cfg: &ConfigSpace, ptr: usize) -> Option<Self> {
        if ptr.checked_add(MSIX_CAP_LEN)? > CONFIG_SPACE_LEN {
            return None;
        }
        let entries = (cfg.u16_at(ptr + MSIX_CONTROL)? & TABLE_SIZE_MASK) as u64 + 1;
        let table = cfg.u32_at(ptr + MSIX_TABLE)?;
        let pba = cfg.u32_at(ptr + MSIX_PBA)?;
        let table_start = (table & !BIR_MASK) as u64;
        let pba_start = (pba & !BIR_MASK) as u64;
        let pba_bytes = entries.div_ceil(64) * 8;
        Some(Self {
            table_bar: (table & BIR_MASK) as u8,
            table: (table_start, table_start + entries * ENTRY_BYTES),
            pba_bar: (pba & BIR_MASK) as u8,
            pba: (pba_start, pba_start + pba_bytes),
        })
    }

    /// Whether the whole pages `[start, end)` of `bar` reach the table or
    /// the PBA. `start` and `end` are page aligned (a region's page span).
    pub fn blocks(&self, bar: u8, start: u64, end: u64) -> bool {
        let hits = |range: (u64, u64)| start < range.1 && range.0 < end;
        (bar == self.table_bar && hits(self.table)) || (bar == self.pba_bar && hits(self.pba))
    }
}
