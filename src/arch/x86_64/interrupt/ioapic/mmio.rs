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
use crate::memory::layout::PAGE_SIZE;

pub(crate) unsafe fn map_mmio(pa: PhysAddr) -> IoApicResult<VirtAddr> {
    crate::memory::mmio::map_device_memory(pa, PAGE_SIZE).map_err(|_| IoApicError::MmioMapFailed)
}

/*
 * The IOAPIC is reached through an index register and a data window, so each
 * access is two MMIO operations and updating a redirection entry takes four
 * behind a read of both halves. Two CPUs interleaving them read or write the
 * wrong register. With the broker masking a line in its interrupt on one CPU
 * while a driver acknowledged on another, the virtio-blk entry was found with
 * its low word copied into its high word and its remote IRR stuck set, and
 * the line never fired again. One lock covers every access, taken with
 * interrupts masked because the broker takes it from interrupt context.
 */
static REG_LOCK: spin::Mutex<()> = spin::Mutex::new(());

/// Run `f` holding the IOAPIC register lock with interrupts masked. Every
/// index/window access to any IOAPIC in the kernel goes through here.
pub(crate) fn locked<R>(f: impl FnOnce() -> R) -> R {
    crate::arch::run_without_interrupts(|| {
        let _guard = REG_LOCK.lock();
        f()
    })
}

#[inline(always)]
fn raw_write(base: VirtAddr, index: u32, val: u32) {
    unsafe {
        let sel = (base.as_u64() + IOREGSEL) as *mut u32;
        let win = (base.as_u64() + IOWIN) as *mut u32;
        core::ptr::write_volatile(sel, index);
        core::ptr::write_volatile(win, val);
    }
}

#[inline(always)]
fn raw_read(base: VirtAddr, index: u32) -> u32 {
    unsafe {
        let sel = (base.as_u64() + IOREGSEL) as *mut u32;
        let win = (base.as_u64() + IOWIN) as *const u32;
        core::ptr::write_volatile(sel, index);
        core::ptr::read_volatile(win)
    }
}

pub(crate) fn reg_read(base: VirtAddr, index: u32) -> u32 {
    locked(|| raw_read(base, index))
}

pub(crate) unsafe fn redtbl_write(base: VirtAddr, i: u32, low: u32, high: u32) {
    locked(|| {
        raw_write(base, IOREDTBL0 + (i * 2) + 1, high);
        raw_write(base, IOREDTBL0 + (i * 2), low);
    })
}

pub(crate) unsafe fn redtbl_read(base: VirtAddr, i: u32) -> (u32, u32) {
    locked(|| {
        let high = raw_read(base, IOREDTBL0 + (i * 2) + 1);
        let low = raw_read(base, IOREDTBL0 + (i * 2));
        (low, high)
    })
}

/// Read entry `i`, let `f` compute its new (low, high) and write it back, all
/// under one hold of the lock so no other update lands in between.
pub(crate) unsafe fn redtbl_update(base: VirtAddr, i: u32, f: impl FnOnce(u32, u32) -> (u32, u32)) {
    locked(|| {
        let high = raw_read(base, IOREDTBL0 + (i * 2) + 1);
        let low = raw_read(base, IOREDTBL0 + (i * 2));
        let (low, high) = f(low, high);
        raw_write(base, IOREDTBL0 + (i * 2) + 1, high);
        raw_write(base, IOREDTBL0 + (i * 2), low);
    })
}
