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

/// The output address and level of the entry that maps `virt` under `root`,
/// reaching each next table through `table_at`, which turns a table's
/// physical address into the table: the kernel answers with
/// `pointer::ram`, a host proof with its own memory.
pub fn walk_page_tables<'a>(
    root: &'a PageTable,
    virt: u64,
    granule: Granule,
    table_at: impl Fn(u64) -> Option<&'a PageTable>,
) -> Option<(u64, usize)> {
    let mut table = root;
    let levels = granule.levels();
    for level in 0..levels {
        let index = granule.index_at_level(virt, level);
        let entry = table.entry(index);
        if entry & PTE_VALID == 0 {
            return None;
        }
        if level == levels - 1 || (entry & PTE_TABLE == 0) {
            return Some((entry & PTE_ADDR_MASK, level));
        }
        table = table_at(entry & PTE_ADDR_MASK)?;
    }
    None
}
