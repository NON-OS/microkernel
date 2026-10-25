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

//! The one device table every unit of a PCI segment is pointed at, so a
//! device's entry is the same whichever unit sees its requests (48882,
//! 2.2.2): 2 MiB, an entry for every requester id, each blocked until
//! written, so unknown devices are denied.

use core::sync::atomic::{AtomicU64, Ordering};

use super::super::dte::blocked;
use super::super::error::AmdViError;
use super::super::regs::DEV_TABLE_PAGES;
use crate::arch::x86_64::iommu::tables::view::Frame;
use crate::memory::phys::{alloc_contiguous, AllocFlags};

static TABLE: AtomicU64 = AtomicU64::new(0);

/// Entries per 4 KiB page: four quadwords each.
pub(super) const PER_PAGE: usize = 128;
/// The table, allocated and filled with blocked entries on first use.
pub(in super::super) fn device_table() -> Result<u64, AmdViError> {
    let existing = TABLE.load(Ordering::Acquire);
    if existing != 0 {
        return Ok(existing);
    }
    let phys = alloc_contiguous(DEV_TABLE_PAGES, AllocFlags::ZERO).ok_or(AmdViError::NoFrames)?;
    for page in 0..DEV_TABLE_PAGES {
        let words =
            Frame::at(phys + page as u64 * 4096).map_err(|_| AmdViError::TableUnreachable)?;
        for entry in 0..PER_PAGE {
            for (word, value) in blocked().into_iter().enumerate() {
                words.write(entry * 4 + word, value)?;
            }
        }
    }
    TABLE.store(phys, Ordering::Release);
    Ok(phys)
}
