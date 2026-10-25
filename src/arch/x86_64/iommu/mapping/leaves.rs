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

//! A checked range's leaves written and committed through iommu::place, so
//! a failure is taken back before a frame is freed, or reported Stranded.

use core::cell::{Cell, RefCell};

use super::commit::commit;
use crate::arch::x86_64::iommu::globals::snoop_control;
use crate::arch::x86_64::iommu::place::{place, Unplaced};
use crate::arch::x86_64::iommu::tables::sl_pte::leaf;
use crate::arch::x86_64::iommu::tables::touched::Touched;
use crate::arch::x86_64::iommu::tables::view::Frame;
use crate::arch::x86_64::iommu::tables::walk::{walk_create, walk_lookup};
use crate::arch::x86_64::iommu::types::{DomainId, IommuPageFlags, VtdError, PAGE_SIZE_4K};

/// `pages` leaves from `iova` to `phys`, in `domain`'s tables under `root`.
pub(super) fn leaves(
    (root, levels): (u64, u8),
    domain: DomainId,
    iova: u64,
    phys: u64,
    pages: usize,
    flags: IommuPageFlags,
) -> Result<(), VtdError> {
    let touched = RefCell::new(Touched::default());
    // Once a leaf is cleared the flush is an unmap's, owed in every mode.
    let cleared = Cell::new(false);
    let at = |page: u64| iova + page * PAGE_SIZE_4K as u64;
    let write = |page: u64| {
        let slot = walk_create(root, at(page), levels)?;
        let snoop = flags.snoop && snoop_control();
        let target = phys + page * PAGE_SIZE_4K as u64;
        let entry = leaf(target, flags.read, flags.write, snoop);
        Frame::at(slot.table_phys)?.write(slot.index, entry)?;
        touched.borrow_mut().note(slot.table_phys);
        Ok(())
    };
    let clear = |page: u64| {
        let Ok(Some(slot)) = walk_lookup(root, at(page), levels) else { return false };
        let Ok(entries) = Frame::at(slot.table_phys) else { return false };
        cleared.set(true);
        touched.borrow_mut().note(slot.table_phys);
        entries.write(slot.index, 0).is_ok()
    };
    let flush = || commit(core::mem::take(&mut *touched.borrow_mut()), domain, !cleared.get());
    place(pages as u64, write, clear, flush).map_err(|u| match u {
        Unplaced::Failed(e) => e,
        Unplaced::Stranded => VtdError::Stranded,
    })
}
