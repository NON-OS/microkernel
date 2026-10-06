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

//! Byte and word access to one register of the window, each refused when it
//! would reach past the mapping.

use super::map::{window, TPM_MMIO_SIZE};
use crate::security::tpm::error::TpmError;

/// Address of a register inside the window, refused rather than computed when
/// the access would reach past the mapping.
fn register(offset: u32, width: usize) -> Result<usize, TpmError> {
    let base = window()?;
    if (offset as usize) + width > TPM_MMIO_SIZE {
        return Err(TpmError::InvalidResponse);
    }
    Ok(base as usize + offset as usize)
}

pub(in crate::security::tpm) fn read8(offset: u32) -> Result<u8, TpmError> {
    let at = register(offset, 1)?;
    // SAFETY: eK@nonos.systems - `at` lies inside the live uncached device
    // mapping made by `init_window`; a byte read of a TPM register has no
    // side effect beyond the FIFO data port, whose caller owns the sequencing.
    Ok(unsafe { core::ptr::read_volatile(at as *const u8) })
}

pub(in crate::security::tpm) fn read32(offset: u32) -> Result<u32, TpmError> {
    let at = register(offset, 4)?;
    // SAFETY: eK@nonos.systems - as `read8`; TPM registers tolerate an aligned
    // 32-bit read.
    Ok(unsafe { core::ptr::read_volatile(at as *const u32) })
}

/// # Safety
/// Writing a TPM register starts, feeds or cancels a command. The caller owns
/// the sequencing the part expects around it.
pub(in crate::security::tpm) unsafe fn write8(offset: u32, value: u8) -> Result<(), TpmError> {
    let at = register(offset, 1)?;
    // SAFETY: eK@nonos.systems - inside the mapping, as `read8`; the caller
    // promised the sequencing.
    unsafe { core::ptr::write_volatile(at as *mut u8, value) };
    Ok(())
}

/// # Safety
/// As [`write8`].
pub(in crate::security::tpm) unsafe fn write32(offset: u32, value: u32) -> Result<(), TpmError> {
    let at = register(offset, 4)?;
    // SAFETY: eK@nonos.systems - inside the mapping, as `read32`; the caller
    // promised the sequencing.
    unsafe { core::ptr::write_volatile(at as *mut u32, value) };
    Ok(())
}
