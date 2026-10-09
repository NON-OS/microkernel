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

//! Loader-data pages: given back to the firmware unless kept for the kernel.

use uefi::prelude::*;
use uefi::table::boot::{AllocateType, MemoryType};

pub(super) const PAGE: usize = 4096;

/* Loader-data pages, given back unless kept. */
pub(super) struct Pages<'a> {
    bs: &'a BootServices,
    base: u64,
    count: usize,
}

impl<'a> Pages<'a> {
    pub(super) fn new(bs: &'a BootServices, count: usize) -> Option<Self> {
        let base =
            bs.allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, count).ok()?;
        Some(Self { bs, base, count })
    }

    pub(super) fn bytes(&self) -> &'a mut [u8] {
        /* SAFETY: the firmware just gave these pages to this loader alone,
         * and they stay allocated for as long as `self` or the kept copy. */
        unsafe { core::slice::from_raw_parts_mut(self.base as *mut u8, self.count * PAGE) }
    }

    pub(super) fn keep(self) -> u64 {
        let base = self.base;
        core::mem::forget(self);
        base
    }
}

impl Drop for Pages<'_> {
    fn drop(&mut self) {
        let _ = self.bs.free_pages(self.base, self.count);
    }
}
