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

//! Waiting for a submitted batch: the unit writes the wait descriptor's
//! sequence into the status dword once everything before it completed.

use super::error::take_error;
use crate::arch::x86_64::iommu::regs::offsets;
use crate::arch::x86_64::iommu::tables::frame::entries_mut;
use crate::arch::x86_64::iommu::types::VtdError;
use crate::arch::x86_64::iommu::unit::access::RemapUnit;
use crate::arch::x86_64::iommu::unit::clock::wait_ms;

/// A queue error ends the wait with that error; silence for `COMMAND_MS`
/// ends it with `Timeout`.
pub(super) fn finish(
    unit: &RemapUnit,
    ring_phys: u64,
    status_phys: u64,
    sequence: u32,
) -> Result<(), VtdError> {
    let status = entries_mut(status_phys)?.as_ptr() as *const u32;
    let mut refused = None;
    let finished = wait_ms(offsets::COMMAND_MS, || {
        if refused.is_none() {
            refused = take_error(unit, ring_phys, status_phys, sequence);
        }
        // SAFETY: eK@nonos.systems - the first dword of the status page this
        // module owns; the unit writes it, so it is read volatile.
        unsafe { core::ptr::read_volatile(status) == sequence }
    });
    match (refused, finished) {
        (Some(e), _) => Err(e),
        (None, true) => Ok(()),
        (None, false) => Err(VtdError::Timeout),
    }
}
