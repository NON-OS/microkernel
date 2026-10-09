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

//! The record, found once and every range in it checked under the directmap.

use spin::Once;

use super::record::{Extent, Record, EXTENTS, LEN, MAGIC};
use crate::boot::handoff::types::MODULE_KIND_DISK_MIRROR;
use crate::memory::addr::PhysAddr;
use crate::memory::unified::phys_to_virt;
use crate::syscall::microkernel::install_source_modules::find;

/* The disk's size in sectors and each range, with its virtual address. */
pub(super) struct Mirror {
    pub(super) capacity: u64,
    pub(super) count: usize,
    pub(super) extents: [(Extent, u64); EXTENTS],
}

static MIRROR: Once<Option<Mirror>> = Once::new();

pub(super) fn mirror() -> Option<&'static Mirror> {
    MIRROR.call_once(load).as_ref()
}

fn load() -> Option<Mirror> {
    let m = find(MODULE_KIND_DISK_MIRROR as u64).filter(|m| m.size == LEN as u64)?;
    let at = mapped(m.base, LEN as u64)?;
    // SAFETY: eK@nonos.systems - the loader wrote the record into loader
    // memory, which the kernel never frees, and both its ends were just
    // checked to lie under the directmap.
    let r = unsafe { core::ptr::read_unaligned(at as *const Record) };
    let count = usize::try_from(r.count).ok().filter(|&c| c > 0 && c <= EXTENTS)?;
    if r.magic != MAGIC {
        return None;
    }
    let mut out =
        Mirror { capacity: r.capacity, count, extents: [(Extent::default(), 0); EXTENTS] };
    for (slot, e) in out.extents.iter_mut().zip(&r.extents[..count]) {
        *slot = (*e, mapped(e.phys, e.sectors.checked_mul(512)?)?);
    }
    let line = alloc::format!("[BLOCK] the loader's copy of {} boot disk ranges", count);
    crate::sys::serial::println(line.as_bytes());
    Some(out)
}

/* The virtual address of `len` bytes from physical `base`, both ends mapped. */
fn mapped(base: u64, len: u64) -> Option<u64> {
    let last = base.checked_add(len.checked_sub(1)?)?;
    phys_to_virt(PhysAddr::new(last))?;
    Some(phys_to_virt(PhysAddr::new(base))?.as_u64())
}
