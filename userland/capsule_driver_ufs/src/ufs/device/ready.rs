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

//! The device's two readiness checks, in the order JESD220 sets them (as
//! recalled, unverified): the UTP layer answers a NOP OUT, then the host
//! sets fDeviceInit and the device clears it once its own initialisation
//! is through.

use super::ask::{ask, Ask};
use crate::ufs::ctx::Ctx;
use crate::ufs::env::{pause, Deadline, Hw};
use crate::ufs::error::UfsError;
use crate::ufs::facts::bounds::{DEVICE_INIT_MS, DEVICE_INIT_POLL_MS};
use crate::ufs::facts::upiu::*;

pub fn answers<H: Hw>(hw: &H, c: &Ctx) -> Result<(), UfsError> {
    ask(hw, c, Ask::Nop).map(|_| ())
}

pub fn initialised<H: Hw>(hw: &H, c: &Ctx) -> Result<(), UfsError> {
    let flag = |func, op| Ask::Flag { func, op, idn: FLAG_DEVICE_INIT };
    ask(hw, c, flag(QUERY_WRITE, OP_SET_FLAG))?;
    let d = Deadline::after(hw, DEVICE_INIT_MS);
    while ask(hw, c, flag(QUERY_READ, OP_READ_FLAG))? {
        if d.passed(hw) {
            return Err(UfsError::Timeout);
        }
        pause(hw, DEVICE_INIT_POLL_MS);
    }
    Ok(())
}
