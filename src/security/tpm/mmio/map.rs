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

pub(in crate::security::tpm) const TPM_MMIO_BASE: u64 = 0xFED4_0000;
/// Localities 0 through 4, one page each.
pub(super) const TPM_MMIO_SIZE: usize = 0x5000;

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
    let va = map_device_memory(PhysAddr::new(TPM_MMIO_BASE), TPM_MMIO_SIZE)
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
