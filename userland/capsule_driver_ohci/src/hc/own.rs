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

//! Taking the controller from firmware. An SMM driver that owns it (the
//! InterruptRouting bit set) is asked to give it up with an ownership
//! change request and waited for; firmware's legacy keyboard emulation,
//! where the controller has it, is turned off.

use crate::env::{until, Clock, Regs};
use crate::facts::bounds::OWNERSHIP_MS;
use crate::facts::regs::{CS_OCR, CTL_IR, HCE_CONTROL, HC_COMMAND_STATUS, HC_CONTROL};
use crate::facts::regs::{HC_REVISION, REV_LEGACY};

pub const SMM_KEPT: &str = "firmware's SMM driver kept the controller";

/// `mapped` is how many register bytes the grant covers: HceControl lies
/// past the operational registers.
pub fn take<R: Regs, C: Clock>(r: &mut R, c: &C, mapped: u64) -> Result<(), &'static str> {
    if r.r32(HC_CONTROL) & CTL_IR != 0 {
        r.w32(HC_COMMAND_STATUS, CS_OCR);
        if !until(c, OWNERSHIP_MS, || r.r32(HC_CONTROL) & CTL_IR == 0) {
            return Err(SMM_KEPT);
        }
    }
    if r.r32(HC_REVISION) & REV_LEGACY != 0 && mapped > HCE_CONTROL as u64 {
        r.w32(HCE_CONTROL, 0);
    }
    Ok(())
}
