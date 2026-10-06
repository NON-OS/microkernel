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

use super::super::error::{IoApicError, IoApicResult};
use super::super::mmio::redtbl_update;
use super::super::ops_helpers::locate;

pub fn retarget(gsi: u32, dest_apic_id: u32) -> IoApicResult<()> {
    let (chip, idx) = locate(gsi).ok_or(IoApicError::GsiNotFound)?;
    // The RTE destination field is 8 bits wide. An APIC id that does not fit
    // is not truncated onto some other CPU; the line goes to one that fits.
    let dest_apic_id = crate::arch::x86_64::interrupt::apic::device_irq_dest(dest_apic_id)
        .ok_or(IoApicError::NoReachableCpu)?;
    unsafe {
        redtbl_update(chip.mmio, idx, |low, high| {
            (low, (high & !(0xFF << 24)) | ((dest_apic_id & 0xFF) << 24))
        });
    }
    Ok(())
}
