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

//! The one device table every unit is pointed at, shared as Linux shares one
//! per PCI segment (amd/init.c, pci_seg->dev_table): 2 MiB, an entry for every
//! requester id, each blocked until written, so unknown devices are denied.

use core::sync::atomic::{compiler_fence, AtomicU64, Ordering};

use super::dte::{blocked, Dte};
use super::error::AmdViError;
use super::regs::DEV_TABLE_PAGES;
use crate::arch::x86_64::iommu::tables::frame::entries_mut;
use crate::memory::phys::{alloc_contiguous, AllocFlags};

static TABLE: AtomicU64 = AtomicU64::new(0);

/// Entries per 4 KiB page: four quadwords each.
const PER_PAGE: usize = 128;

/// The table, allocated and filled with blocked entries on first use.
pub(super) fn device_table() -> Result<u64, AmdViError> {
    let existing = TABLE.load(Ordering::Acquire);
    if existing != 0 {
        return Ok(existing);
    }
    let phys = alloc_contiguous(DEV_TABLE_PAGES, AllocFlags::ZERO).ok_or(AmdViError::NoFrames)?;
    for page in 0..DEV_TABLE_PAGES {
        let words =
            entries_mut(phys + page as u64 * 4096).map_err(|_| AmdViError::TableUnreachable)?;
        for entry in words.chunks_exact_mut(4) {
            entry.copy_from_slice(&blocked());
        }
    }
    TABLE.store(phys, Ordering::Release);
    Ok(phys)
}

/// Replace the entry for `device_id`, the first quadword (V, TV, mode and
/// root) last, so a unit never reads a new root beside an old domain. The
/// caller flushes the unit's cached copy afterwards.
pub(super) fn write_dte(device_id: u16, entry: Dte) -> Result<(), AmdViError> {
    let (words, at) = slot_of(device_id)?;
    words[at + 3] = entry[3];
    words[at + 2] = entry[2];
    words[at + 1] = entry[1];
    compiler_fence(Ordering::SeqCst);
    words[at] = entry[0];
    Ok(())
}

pub(super) fn read_dte(device_id: u16) -> Result<Dte, AmdViError> {
    let (words, at) = slot_of(device_id)?;
    Ok([words[at], words[at + 1], words[at + 2], words[at + 3]])
}

/// The page holding `device_id`'s entry, and the entry's first quadword in it.
fn slot_of(device_id: u16) -> Result<(&'static mut [u64], usize), AmdViError> {
    let page = device_table()? + (device_id as usize / PER_PAGE) as u64 * 4096;
    let words = entries_mut(page).map_err(|_| AmdViError::TableUnreachable)?;
    Ok((words, (device_id as usize % PER_PAGE) * 4))
}
