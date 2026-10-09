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

//! The kernel's [`RegisterBus`] / [`ResetBus`]: real port I/O, physical
//! memory through a short-lived uncached device mapping, PCI configuration
//! space through the PCI config accessors, and a busy wait that needs no
//! interrupts and no HPET.

use super::gas::RegisterBus;
use super::reset::ResetBus;
use crate::arch::x86_64::port::{inb, inl, inw, outb, outl, outw};
use crate::memory::addr::{PhysAddr, VirtAddr};

const PAGE: u64 = 4096;

pub struct PortBus;

impl PortBus {
    /// Map the page holding `phys`, run `f` on the virtual address of
    /// `phys`, unmap. Firmware registers live in MMIO that the direct map
    /// does not promise to cover (and would cover cached), so a device
    /// mapping is made for each access; these paths run a handful of times
    /// per boot at most.
    fn with_mapped<R>(phys: u64, bits: u8, f: impl FnOnce(u64) -> R) -> Option<R> {
        let bytes = (bits / 8) as u64;
        if bytes == 0 || phys % bytes != 0 {
            return None;
        }
        let page = phys & !(PAGE - 1);
        let span = if (phys - page) + bytes > PAGE { 2 * PAGE } else { PAGE };
        let va = crate::memory::mmio::map_device_memory(PhysAddr::new(page), span as usize).ok()?;
        let r = f(va.as_u64() + (phys - page));
        let _ = crate::memory::mmio::unmap_mmio(VirtAddr::new(va.as_u64()));
        Some(r)
    }
}

impl RegisterBus for PortBus {
    fn read_io(&mut self, port: u16, bits: u8) -> u32 {
        // SAFETY: the port comes from the firmware's FADT or is one of the
        // fixed PC reset ports; reads of these have no side effect beyond
        // the documented register semantics.
        unsafe {
            match bits {
                8 => inb(port) as u32,
                16 => inw(port) as u32,
                _ => inl(port),
            }
        }
    }

    fn write_io(&mut self, port: u16, bits: u8, value: u32) {
        // SAFETY: as for `read_io`; the write is the register access the
        // caller asked for.
        unsafe {
            match bits {
                8 => outb(port, value as u8),
                16 => outw(port, value as u16),
                _ => outl(port, value),
            }
        }
    }

    fn read_mem(&mut self, phys: u64, bits: u8) -> Option<u64> {
        Self::with_mapped(phys, bits, |va| {
            // SAFETY: `va` is a fresh uncached mapping of `phys`, aligned to
            // the access width (checked in `with_mapped`).
            unsafe {
                match bits {
                    8 => core::ptr::read_volatile(va as *const u8) as u64,
                    16 => core::ptr::read_volatile(va as *const u16) as u64,
                    32 => core::ptr::read_volatile(va as *const u32) as u64,
                    _ => core::ptr::read_volatile(va as *const u64),
                }
            }
        })
    }

    fn write_mem(&mut self, phys: u64, bits: u8, value: u64) -> bool {
        Self::with_mapped(phys, bits, |va| {
            // SAFETY: as for `read_mem`.
            unsafe {
                match bits {
                    8 => core::ptr::write_volatile(va as *mut u8, value as u8),
                    16 => core::ptr::write_volatile(va as *mut u16, value as u16),
                    32 => core::ptr::write_volatile(va as *mut u32, value as u32),
                    _ => core::ptr::write_volatile(va as *mut u64, value),
                }
            }
        })
        .is_some()
    }
}

impl ResetBus for PortBus {
    fn pci_write8(&mut self, device: u8, function: u8, offset: u16, value: u8) -> bool {
        crate::drivers::pci::config::write8(0, device, function, offset, value).is_ok()
    }

    fn delay_us(&mut self, us: u32) {
        busy_wait_us(us);
    }
}

/// Busy wait on the TSC when its rate is known, else on port 0x80 reads,
/// each of which costs about a microsecond on the LPC/eSPI bus (the same
/// delay Linux's `io_delay` uses). Neither needs interrupts, the HPET or the
/// PIT, which a laptop may have gated off.
pub fn busy_wait_us(us: u32) {
    let hz = crate::sys::timer::tsc::tsc_frequency();
    if hz != 0 {
        let ticks = (hz as u128 * us as u128 / 1_000_000) as u64;
        let start = crate::sys::timer::tsc::rdtsc();
        while crate::sys::timer::tsc::rdtsc().wrapping_sub(start) < ticks {
            core::hint::spin_loop();
        }
        return;
    }
    for _ in 0..us {
        // SAFETY: port 0x80 is the POST diagnostic port; reading it has no
        // effect beyond the bus cycle.
        unsafe {
            let _ = inb(0x80);
        }
    }
}
