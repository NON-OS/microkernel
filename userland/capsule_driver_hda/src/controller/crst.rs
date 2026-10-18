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

//! The controller reset bit, GCTL.CRST, as a handshake: written, then waited
//! on until it reads back, by elapsed time (HDA 1.0a section 3.3.7).

use super::reset::RESET_MS;
use super::wait::until;
use crate::constants::{GCTL, GCTL_CRST};
use crate::error::{HdaError, HdaResult};
use crate::regs::Regs;

/// Clear CRST, preserving the rest of GCTL, and wait for it to read back.
pub fn enter_reset(regs: Regs) -> HdaResult<()> {
    crst(regs, false)
}

/// Set CRST, preserving the rest of GCTL, and wait for it to read back.
pub fn leave_reset(regs: Regs) -> HdaResult<()> {
    crst(regs, true)
}

fn crst(regs: Regs, set: bool) -> HdaResult<()> {
    let want = if set { GCTL_CRST } else { 0 };
    // SAFETY: GCTL lies in every HDA register window (section 3.3).
    unsafe { regs.w32(GCTL, (regs.r32(GCTL) & !GCTL_CRST) | want) };
    // SAFETY: as above.
    let seen = until(RESET_MS, || unsafe { regs.r32(GCTL) } & GCTL_CRST == want);
    seen.then_some(()).ok_or(HdaError::ControllerResetTimeout)
}
