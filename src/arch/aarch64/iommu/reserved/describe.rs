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

//! Reading the tree's reserved DMA ranges at boot, while the tree is there.

use super::table::{Table, MAX_REGIONS, TABLE};
use crate::arch::fdt::find::reserved_dma::{ranges, Refusal, TreeRange, MAX_TREE_RANGES};
use crate::arch::fdt::{Fdt, FdtError};

/// Keep the entries and the regions. An entry no device can be keyed by is
/// said here, once, as is each entry that cannot be a range (it refuses its
/// device's claim later). A tree whose entries cannot be read keeps none,
/// and says why: a device the tree reserved a range for is then attached
/// without it, as before this was read.
pub fn describe(fdt: &Fdt) -> Result<usize, Refusal> {
    let mut entries = [TreeRange::EMPTY; MAX_TREE_RANGES];
    let mut unkeyed = None;
    let count = ranges(fdt, &mut entries, &mut unkeyed)?;
    let mut regions = [(0u64, 0u64); MAX_REGIONS];
    let region_count = fdt.reserved_memory(&mut regions).map_err(|e| match e {
        FdtError::TooLarge => Refusal::TooMany,
        other => Refusal::Tree(other),
    })?;
    let say = |what: &[u8], why: &[u8]| {
        crate::sys::serial::Line::new().str(b"[IOMMU] reserved DMA: ").str(what).str(why).end();
    };
    if let Some(why) = unkeyed {
        say(b"an iommu-addresses entry left out: ", why);
    }
    for entry in entries[..count].iter().filter_map(|e| e.problem) {
        say(b"its device's claim will be refused: ", entry);
    }
    TABLE.call_once(|| Table { entries, count, regions, region_count });
    Ok(count)
}
