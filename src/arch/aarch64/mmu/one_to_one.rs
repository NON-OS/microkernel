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

//! Whether code can run one to one under a root: what a core coming back
//! from a powerdown idle state needs between turning its MMU on and leaving
//! for the kernel's own address (asm/suspend.S).

use super::attributes::{PTE_ADDR_MASK, PTE_AF, PTE_PXN};
use super::granule::Granule;
use super::table::{leaf_entry, PageTable};

/// Every byte of `[start, end)` translated under `root` to itself, by an
/// entry with its access flag set and EL1 execution allowed; or why not.
/// `table_at` as for `walk_page_tables`; the kernel's granule is 4 KiB.
pub fn executable_one_to_one<'a>(
    root: &'a PageTable,
    start: u64,
    end: u64,
    table_at: impl Fn(u64) -> Option<&'a PageTable>,
) -> Result<(), &'static [u8]> {
    let granule = Granule::G4K;
    let mut at = start;
    while at < end {
        let (entry, level) = leaf_entry(root, at, granule, &table_at).ok_or(&b"not mapped"[..])?;
        let size = match level {
            _ if level == granule.levels() - 1 => granule.page_size(),
            _ => granule.block_size(level).ok_or(&b"a block at a level that has none"[..])?,
        } as u64;
        if entry & PTE_AF == 0 {
            return Err(b"mapped without its access flag");
        }
        if entry & PTE_PXN != 0 {
            return Err(b"mapped but not executable at EL1");
        }
        if (entry & PTE_ADDR_MASK & !(size - 1)) | (at & (size - 1)) != at {
            return Err(b"mapped, but not one to one");
        }
        // On to the next entry, or past the top of memory and done.
        match (at | (size - 1)).checked_add(1) {
            Some(next) => at = next,
            None => break,
        }
    }
    Ok(())
}
