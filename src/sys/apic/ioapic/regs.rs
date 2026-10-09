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

use core::sync::atomic::Ordering;

use super::constants::IOAPIC_WIN;
use super::state::IOAPIC_BASE;
use crate::arch::x86_64::interrupt::ioapic::reg_lock::locked;

/*
 * SAFETY: caller has confirmed the IOAPIC was discovered and the MMIO
 * base is mapped uncached in kernel space. Two volatile MMIO ops per
 * register access, made under the IOAPIC register lock so the index and
 * the window stay paired; a sequence of accesses is not atomic as a whole.
 */
pub(super) unsafe fn ioapic_read(reg: u32) -> u32 {
    unsafe {
        let base = IOAPIC_BASE.load(Ordering::Relaxed);
        let regsel = base as *mut u32;
        let window = (base + IOAPIC_WIN as u64) as *mut u32;
        /*
         * The same chip the broker programs: share its register lock so the
         * index and the window are never split by another CPU.
         */
        locked(|| {
            core::ptr::write_volatile(regsel, reg);
            core::ptr::read_volatile(window)
        })
    }
}

pub(super) unsafe fn ioapic_write(reg: u32, value: u32) {
    unsafe {
        let base = IOAPIC_BASE.load(Ordering::Relaxed);
        let regsel = base as *mut u32;
        let window = (base + IOAPIC_WIN as u64) as *mut u32;
        locked(|| {
            core::ptr::write_volatile(regsel, reg);
            core::ptr::write_volatile(window, value);
        })
    }
}
