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

//! Mapping the register window, once.

use core::sync::atomic::{AtomicU64, Ordering};

use crate::memory::addr::PhysAddr;
use crate::memory::mmio::map_device_memory;
use crate::security::tpm::error::TpmError;

/// Localities 0 through 4, one page each.
pub(super) const TPM_MMIO_SIZE: usize = 0x5000;

/// The window's physical base, as the platform places it (arch::tpm_window),
/// or `None` with no TPM there. One shorter than the five localities is
/// refused: every register access is bounded by `TPM_MMIO_SIZE`, and would
/// otherwise reach past what the device decodes.
pub(in crate::security::tpm) fn tpm_mmio_base() -> Option<u64> {
    let (base, len) = crate::arch::tpm_window()?;
    (len >= TPM_MMIO_SIZE as u64).then_some(base)
}

/// Virtual address of the mapped register window, or zero before bring-up.
/// The kernel differs from the bootloader here: there is no identity map, so
/// the window has to be mapped before a single register can be read.
static WINDOW: AtomicU64 = AtomicU64::new(0);

/// Map the register window once, uncached. Idempotent, so a second caller
/// reuses the first mapping rather than creating an alias of the same device
/// memory.
pub(in crate::security::tpm) fn init_window() -> Result<u64, TpmError> {
    let existing = WINDOW.load(Ordering::Acquire);
    if existing != 0 {
        return Ok(existing);
    }
    let base = tpm_mmio_base().ok_or(TpmError::NotPresent)?;
    let va = map_device_memory(PhysAddr::new(base), TPM_MMIO_SIZE)
        .map_err(|_| TpmError::NotPresent)?
        .as_u64();
    match WINDOW.compare_exchange(0, va, Ordering::AcqRel, Ordering::Acquire) {
        Ok(_) => Ok(va),
        Err(winner) => Ok(winner),
    }
}

pub(super) fn window() -> Result<u64, TpmError> {
    match WINDOW.load(Ordering::Acquire) {
        0 => Err(TpmError::NotPresent),
        va => Ok(va),
    }
}

/// An uncached mapping of `[phys, phys + size)`, page aligned around it, for the
/// CRB control area and its buffers. It lives here, beside the window map, so
/// the broker gate sees every device mapping the kernel TPM takes in one place.
pub(in crate::security::tpm) fn map_region(phys: u64, size: u64) -> Result<u64, TpmError> {
    const PAGE: u64 = 0x1000;
    let base = phys & !(PAGE - 1);
    let end = phys.checked_add(size).ok_or(TpmError::NotPresent)?;
    let len = ((end - base + PAGE - 1) & !(PAGE - 1)) as usize;
    let va = map_device_memory(PhysAddr::new(base), len).map_err(|_| TpmError::NotPresent)?;
    Ok(va.as_u64() + (phys - base))
}
