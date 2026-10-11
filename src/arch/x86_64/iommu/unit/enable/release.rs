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

//! Taking a unit back from firmware, whose protected memory regions or
//! translation may still be on past ExitBootServices. Both are protection:
//! let go only as iommu::handover says.

use crate::arch::x86_64::iommu::regs::{cap, offsets};
use crate::arch::x86_64::iommu::types::VtdError;
use crate::arch::x86_64::iommu::unit::access::RemapUnit;

/// What firmware had left on.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Released {
    pub protected_regions: bool,
    pub translation: bool,
}

/// True when firmware's protected regions were on and are now off.
///
/// # Safety
/// Widens what devices behind the unit reach: called once the unit
/// translates with the kernel's tables, or under the development opt-in.
pub unsafe fn release_protected_regions(unit: &RemapUnit) -> Result<bool, VtdError> {
    if !cap::has_protected_regions(unit.read64(offsets::CAP))
        || unit.read32(offsets::PMEN) & offsets::PMEN_EPM == 0
    {
        return Ok(false);
    }
    // SAFETY: eK@nonos.systems - as the caller promised above.
    unsafe { unit.write32(offsets::PMEN, 0) };
    wait(|| unit.read32(offsets::PMEN) & offsets::PMEN_PRS == 0)?;
    Ok(true)
}

/// # Safety
/// Turns all protection off: until the kernel programs the unit, devices
/// behind it reach memory directly. The development opt-in only.
pub unsafe fn release_from_firmware(unit: &RemapUnit) -> Result<Released, VtdError> {
    // SAFETY: eK@nonos.systems - the caller's promise, the same.
    let protected_regions = unsafe { release_protected_regions(unit)? };
    let mut released = Released { protected_regions, translation: false };
    let status = unit.read32(offsets::GSTS);
    if status & offsets::GSTS_TES != 0 {
        // SAFETY: eK@nonos.systems - as above; firmware's tables stop being
        // consulted, and its drivers are gone after ExitBootServices.
        unsafe { unit.write32(offsets::GCMD, offsets::gcmd_without_te(status)) };
        wait(|| unit.read32(offsets::GSTS) & offsets::GSTS_TES == 0)?;
        released.translation = true;
    }
    Ok(released)
}

fn wait(done: impl Fn() -> bool) -> Result<(), VtdError> {
    for _ in 0..offsets::COMMAND_SPINS {
        if done() {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(VtdError::Timeout)
}
