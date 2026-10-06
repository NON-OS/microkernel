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

//! Where the CRB control area is, and the part's buffers, as uncached device
//! mappings.
//!
//! The control area is where the TPM2 table says. On Intel's firmware TPM
//! that is 0xFED40040, inside the register window, with the locality
//! registers beside it. On AMD's it is memory of its own and there is no
//! window. The command and response buffers are device memory too, so they
//! are mapped uncached like the window, never reached through the directmap:
//! the directmap's large cacheable pages span the MMIO hole, which leaves the
//! memory type of a buffer there undefined on real hardware.

use core::sync::atomic::{AtomicU64, Ordering};

use spin::Mutex;

use crate::memory::addr::PhysAddr;
use crate::memory::mmio::map_device_memory;
use crate::security::tpm::error::TpmError;
use crate::security::tpm::mmio::{init_window, TPM_MMIO_BASE};
use crate::security::tpm::transport::acpi::control_area;

/// The control area's offset inside the register window.
const WINDOW_CONTROL: u64 = 0x40;
const WINDOW_SIZE: u64 = 0x5000;
const PAGE: u64 = 0x1000;
/// The control area's registers reach 0x30 past its start.
const CONTROL_SPAN: u64 = 0x30;

/// The control area's virtual address once resolved, and whether it sits in
/// the register window (and so has locality registers beside it).
static CONTROL: AtomicU64 = AtomicU64::new(0);
static IN_WINDOW: AtomicU64 = AtomicU64::new(0);

/// The control area's mapping.
pub(super) fn control() -> Result<u64, TpmError> {
    let va = CONTROL.load(Ordering::Acquire);
    if va != 0 {
        return Ok(va);
    }
    let phys = control_area().unwrap_or(TPM_MMIO_BASE + WINDOW_CONTROL);
    let (va, inside) = if in_window(phys, CONTROL_SPAN) {
        (init_window()? + (phys - TPM_MMIO_BASE), 1)
    } else {
        (map(phys, CONTROL_SPAN)?, 2)
    };
    IN_WINDOW.store(inside, Ordering::Release);
    CONTROL.store(va, Ordering::Release);
    Ok(va)
}

/// Whether the control area is the register window's, with locality registers.
pub(super) fn in_register_window() -> Result<bool, TpmError> {
    control()?;
    Ok(IN_WINDOW.load(Ordering::Acquire) == 1)
}

/// A buffer the part published, mapped once and kept: the part publishes the
/// same one or two for its life.
static BUFFERS: Mutex<[(u64, u64, u64); 2]> = Mutex::new([(0, 0, 0); 2]);

pub(super) fn buffer(phys: u64, size: u64) -> Result<u64, TpmError> {
    if in_window(phys, size) {
        return Ok(init_window()? + (phys - TPM_MMIO_BASE));
    }
    let mut kept = BUFFERS.lock();
    if let Some(&(_, _, va)) = kept.iter().find(|&&(p, s, va)| va != 0 && p == phys && s >= size) {
        return Ok(va);
    }
    let va = map(phys, size)?;
    let slot = kept.iter().position(|&(_, _, v)| v == 0).unwrap_or(1);
    kept[slot] = (phys, size, va);
    Ok(va)
}

fn in_window(phys: u64, size: u64) -> bool {
    phys >= TPM_MMIO_BASE && phys.saturating_add(size) <= TPM_MMIO_BASE + WINDOW_SIZE
}

/// An uncached mapping of `[phys, phys + size)`, page aligned around it.
fn map(phys: u64, size: u64) -> Result<u64, TpmError> {
    let base = phys & !(PAGE - 1);
    let end = phys.checked_add(size).ok_or(TpmError::NotPresent)?;
    let len = ((end - base + PAGE - 1) & !(PAGE - 1)) as usize;
    let va = map_device_memory(PhysAddr::new(base), len).map_err(|_| TpmError::NotPresent)?;
    Ok(va.as_u64() + (phys - base))
}

/// One control register, at `offset` from the control area's start.
pub(super) fn read(offset: u32) -> Result<u32, TpmError> {
    let at = control()? + offset as u64;
    // SAFETY: eK@nonos.systems - inside the uncached control area mapping,
    // whose span covers every register offset this driver names.
    Ok(unsafe { core::ptr::read_volatile(at as *const u32) })
}

/// # Safety
/// Writing a control register asks the part to change state; the caller owns
/// the sequence.
pub(super) unsafe fn write(offset: u32, value: u32) -> Result<(), TpmError> {
    let at = control()? + offset as u64;
    // SAFETY: eK@nonos.systems - as `read`; the caller owns the sequence.
    unsafe { core::ptr::write_volatile(at as *mut u32, value) };
    Ok(())
}
