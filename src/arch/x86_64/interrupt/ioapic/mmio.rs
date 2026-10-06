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

use crate::memory::addr::{PhysAddr, VirtAddr};

use super::constants::*;
use super::error::{IoApicError, IoApicResult};
use super::reg_lock::locked;
use crate::memory::layout::PAGE_SIZE;

pub(crate) unsafe fn map_mmio(pa: PhysAddr) -> IoApicResult<VirtAddr> {
    crate::memory::mmio::map_device_memory(pa, PAGE_SIZE).map_err(|_| IoApicError::MmioMapFailed)
}

#[inline(always)]
fn raw_write(base: VirtAddr, index: u32, val: u32) {
    unsafe {
        core::ptr::write_volatile((base.as_u64() + IOREGSEL) as *mut u32, index);
        core::ptr::write_volatile((base.as_u64() + IOWIN) as *mut u32, val);
    }
}

#[inline(always)]
fn raw_read(base: VirtAddr, index: u32) -> u32 {
    unsafe {
        core::ptr::write_volatile((base.as_u64() + IOREGSEL) as *mut u32, index);
        core::ptr::read_volatile((base.as_u64() + IOWIN) as *const u32)
    }
}

pub(crate) fn reg_read(base: VirtAddr, index: u32) -> u32 {
    locked(|| raw_read(base, index))
}

fn raw_entry(base: VirtAddr, i: u32) -> (u32, u32) {
    let high = raw_read(base, IOREDTBL0 + (i * 2) + 1);
    (raw_read(base, IOREDTBL0 + (i * 2)), high)
}

fn raw_set(base: VirtAddr, i: u32, low: u32, high: u32) {
    raw_write(base, IOREDTBL0 + (i * 2) + 1, high);
    raw_write(base, IOREDTBL0 + (i * 2), low);
}

pub(crate) unsafe fn redtbl_write(base: VirtAddr, i: u32, low: u32, high: u32) {
    locked(|| raw_set(base, i, low, high))
}

pub(crate) unsafe fn redtbl_read(base: VirtAddr, i: u32) -> (u32, u32) {
    locked(|| raw_entry(base, i))
}

/// Read entry `i`, let `f` compute its new (low, high) and write it back, all
/// under one hold of the lock so no other update lands in between.
pub(crate) unsafe fn redtbl_update(base: VirtAddr, i: u32, f: impl FnOnce(u32, u32) -> (u32, u32)) {
    locked(|| {
        let (low, high) = raw_entry(base, i);
        let (low, high) = f(low, high);
        raw_set(base, i, low, high);
    })
}
