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

use core::sync::atomic::{compiler_fence, Ordering};

use crate::arch::x86_64::iommu::regs::cap::requires_write_buffer_flush;
use crate::arch::x86_64::iommu::regs::offsets;
use crate::arch::x86_64::iommu::types::VtdError;
use crate::arch::x86_64::iommu::unit::access::RemapUnit;

/// Drain the unit's write buffer when CAP.RWBF says it has one. Such a unit may
/// hold a table write the CPU already made where its walks cannot see it, so an
/// invalidation that follows can refill from the old entry: a mapping that
/// never appears, or an unmap that never happens while the frame is reused.
/// The probe always read the bit; nothing acted on it. A no-op elsewhere,
/// which is every unit QEMU emulates and most recent silicon.
pub(super) fn flush_write_buffer(unit: &RemapUnit) -> Result<(), VtdError> {
    if !requires_write_buffer_flush(unit.read64(offsets::CAP)) {
        return Ok(());
    }
    compiler_fence(Ordering::SeqCst);
    let command = offsets::gcmd_with(unit.read32(offsets::GSTS), offsets::GCMD_WBF);
    // SAFETY: eK@nonos.systems - a write buffer flush moves writes already made
    // to where the unit reads them; it maps and unmaps nothing.
    unsafe {
        unit.write32(offsets::GCMD, command);
    }
    for _ in 0..offsets::COMMAND_SPINS {
        if unit.read32(offsets::GSTS) & offsets::GSTS_WBFS == 0 {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(VtdError::Timeout)
}
