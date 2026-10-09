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

//! Finding the HPET.
//!
//! The only authority is the ACPI HPET table. Many recent laptops have none,
//! or have one Linux force-disables, so absence is normal and every caller
//! has a path without it (the TSC is calibrated from CPUID, the PIT or the
//! ACPI PM timer). The register block is reached through an uncached device
//! mapping made once; the physical address is never dereferenced, and no
//! address is probed on a guess.

use core::sync::atomic::{AtomicU64, Ordering};

use crate::memory::addr::PhysAddr;

/// HPET register block size (IA-PC HPET 1.0a section 2.3.1).
const HPET_MMIO_BYTES: usize = 1024;
/// The spec caps the main counter period at 100 ns (10^8 fs).
const MAX_PERIOD_FS: u64 = 100_000_000;

/// Virtual address of the mapped block, 0 until mapped.
static MAPPED: AtomicU64 = AtomicU64::new(0);

/// Whether a General Capabilities register value describes a real HPET: a
/// nonzero, in-spec counter period and a vendor id that is not all-ones
/// (what an unbacked MMIO read returns).
pub fn capabilities_valid(caps: u64) -> bool {
    let period_fs = caps >> 32;
    let vendor = (caps >> 16) as u16;
    period_fs != 0 && period_fs <= MAX_PERIOD_FS && vendor != 0xFFFF && vendor != 0
}

/// The HPET's mapped register base, or None when the firmware describes no
/// HPET or the block does not answer like one.
pub fn detect_hpet() -> Option<u64> {
    let cached = MAPPED.load(Ordering::Acquire);
    if cached != 0 {
        return Some(cached);
    }
    if !crate::arch::x86_64::acpi::is_initialized() {
        return None;
    }
    let phys = crate::arch::x86_64::acpi::hpet_address()?;
    let va = crate::memory::mmio::map_device_memory(PhysAddr::new(phys), HPET_MMIO_BYTES).ok()?;
    let va = va.as_u64();
    // SAFETY: `va` is a fresh uncached mapping of the HPET block the ACPI
    // table named; offset 0 is the read-only General Capabilities register.
    let caps = unsafe { core::ptr::read_volatile(va as *const u64) };
    if !capabilities_valid(caps) {
        let _ = crate::memory::mmio::unmap_mmio(crate::memory::addr::VirtAddr::new(va));
        return None;
    }
    match MAPPED.compare_exchange(0, va, Ordering::AcqRel, Ordering::Acquire) {
        Ok(_) => Some(va),
        Err(other) => {
            let _ = crate::memory::mmio::unmap_mmio(crate::memory::addr::VirtAddr::new(va));
            Some(other)
        }
    }
}

/// Main counter of a mapped HPET (`base` as returned by [`detect_hpet`]).
pub fn read_hpet_counter(base: u64) -> u64 {
    // SAFETY: `base` is the mapped HPET block; 0xF0 is the main counter.
    unsafe { core::ptr::read_volatile((base + 0xF0) as *const u64) }
}
