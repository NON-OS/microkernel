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

use super::super::attributes::{PTE_ADDR_MASK, PTE_TABLE, PTE_VALID};
use super::super::granule::Granule;
use super::page_table::PageTable;

/// The descriptor that maps `virt` under `root`, and its level.
pub fn leaf_entry<'a>(
    root: &'a PageTable,
    virt: u64,
    granule: Granule,
    table_at: &impl Fn(u64) -> Option<&'a PageTable>,
) -> Option<(u64, usize)> {
    let mut table = root;
    for level in 0..granule.levels() {
        let entry = table.entry(granule.index_at_level(virt, level));
        if entry & PTE_VALID == 0 {
            return None;
        }
        if level == granule.levels() - 1 || entry & PTE_TABLE == 0 {
            return Some((entry, level));
        }
        table = table_at(entry & PTE_ADDR_MASK)?;
    }
    None
}
