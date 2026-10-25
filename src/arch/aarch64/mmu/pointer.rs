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

//! The one file under `src/arch/aarch64` where a number becomes a pointer.
//! Everything else asks here, so what each address means is decided once:
//! `ram` for memory known by its physical address, `device` for a register
//! in a boot device window, `mapped` for a register in a window the kernel
//! mapped and holds the virtual address of. The tree denies the integer to
//! pointer cast everywhere else (`fuzzy_provenance_casts`), and
//! `run-static-checks.sh` counts the few places that still allow it.
#![cfg_attr(target_os = "none", allow(fuzzy_provenance_casts))]

use core::arch::asm;
use core::mem::size_of;

use super::direct_map::direct_map_end;
use super::geometry::KERNEL_SPACE_START;
use super::kernel_va::io;

/// The direct map's one top-level slot, 512 GiB.
const DIRECT_SPAN: u64 = 1 << 39;

fn mmu_on() -> bool {
    let sctlr: u64;
    // SAFETY: reading SCTLR_EL1 at EL1 has no effect.
    unsafe { asm!("mrs {}, sctlr_el1", out(reg) sctlr, options(nomem, nostack, preserves_flags)) };
    sctlr & 1 != 0
}

/// RAM at physical `pa`, as this CPU reaches it now: the address itself
/// while the MMU is off, which is when boot reads what the loader left,
/// else through the direct map in TTBR1, which no user root can take away
/// as it takes the identity map. None where a `T` there would run past the
/// direct map.
pub fn ram<T>(pa: u64) -> Option<*mut T> {
    ram_span(pa, size_of::<T>() as u64).map(|at| at.cast())
}

/// `len` bytes of RAM from physical `pa`, as `ram` reaches them.
pub fn ram_span(pa: u64, len: u64) -> Option<*mut u8> {
    if !mmu_on() {
        return Some(pa as *mut u8);
    }
    let end = pa.checked_add(len)?;
    if end > direct_map_end() || end > DIRECT_SPAN {
        return None;
    }
    Some((KERNEL_SPACE_START + pa) as *mut u8)
}

/// The register `offset` bytes into the boot device window at physical
/// `base`, where the kernel runs now (`io`).
pub fn device<T>(base: u64, offset: u64) -> *mut T {
    io(base).wrapping_add(offset) as *mut T
}

/// The register `offset` bytes into a window the kernel mapped itself and
/// keeps for its lifetime, at virtual `va` (`memory::mmio`).
pub fn mapped<T>(va: u64, offset: u64) -> *mut T {
    va.wrapping_add(offset) as *mut T
}
