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

//! The UIC command port. The host takes a command only while HCS.UCRDY is
//! set; the arguments go in before the opcode, whose write starts it, and
//! IS.UCCS says it is done, with the result in UCMDARG2 and a DME_GET's
//! value in UCMDARG3 (JESD223, as recalled, unverified).

use super::wait::until;
use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;
use crate::ufs::facts::bounds::UIC_MS;
use crate::ufs::facts::hci::{HCS, HCS_UCRDY, IS, IS_UCCS, UCMDARG1, UCMDARG2, UCMDARG3, UICCMD};
use crate::ufs::facts::uic::{mib, DME_GET, DME_LINKSTARTUP, DME_SET, RESULT_MASK};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UicCmd {
    pub op: u32,
    pub args: [u32; 3],
}

impl UicCmd {
    pub const fn link_startup() -> UicCmd {
        UicCmd { op: DME_LINKSTARTUP, args: [0; 3] }
    }

    /// A local attribute, selector 0, normal set type.
    pub const fn get(attr: u16) -> UicCmd {
        UicCmd { op: DME_GET, args: [mib(attr), 0, 0] }
    }

    pub const fn set(attr: u16, value: u32) -> UicCmd {
        UicCmd { op: DME_SET, args: [mib(attr), 0, value] }
    }
}

/// Runs `cmd`; Ok carries UCMDARG3. A completion left over from before is
/// cleared first, so the wait sees only this command's.
pub fn exchange<H: Hw>(hw: &H, cmd: UicCmd) -> Result<u32, UfsError> {
    if !until(hw, UIC_MS, || hw.r32(HCS) & HCS_UCRDY != 0) {
        return Err(UfsError::Timeout);
    }
    hw.w32(IS, IS_UCCS);
    for (reg, v) in [UCMDARG1, UCMDARG2, UCMDARG3].into_iter().zip(cmd.args) {
        hw.w32(reg, v);
    }
    hw.w32(UICCMD, cmd.op);
    if !until(hw, UIC_MS, || hw.r32(IS) & IS_UCCS != 0) {
        return Err(UfsError::Timeout);
    }
    hw.w32(IS, IS_UCCS);
    match hw.r32(UCMDARG2) & RESULT_MASK {
        0 => Ok(hw.r32(UCMDARG3)),
        r => Err(UfsError::Uic(r as u8)),
    }
}
