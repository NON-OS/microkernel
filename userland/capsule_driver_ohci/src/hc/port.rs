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

//! A root port: reset (the root hub times it and says when it is done),
//! enabled by the reset, and its speed; or disabled. Writes carry only the
//! one command or change bit they mean.

use crate::env::{hold, until, Clock, Regs};
use crate::facts::bounds::{PORT_RECOVERY_MS, PORT_RESET_MS};
use crate::facts::port::*;

pub const NO_DEVICE: &str = "nothing connected on the port";
pub const RESET_STUCK: &str = "port reset did not finish";
pub const NOT_ENABLED: &str = "port did not enable after reset";

pub fn offset(port: u8) -> u32 {
    HC_RH_PORT_STATUS + 4 * (port as u32 - 1)
}

/// Reset `port` and see it enabled; true for a low-speed device.
pub fn reset<R: Regs, C: Clock>(r: &mut R, c: &C, port: u8) -> Result<bool, &'static str> {
    let off = offset(port);
    if r.r32(off) & PORT_CCS == 0 {
        return Err(NO_DEVICE);
    }
    r.w32(off, PORT_SET_RESET);
    if !until(c, PORT_RESET_MS, || r.r32(off) & PORT_PRSC != 0) {
        return Err(RESET_STUCK);
    }
    r.w32(off, PORT_PRSC | PORT_PESC);
    if r.r32(off) & PORT_PES == 0 {
        return Err(NOT_ENABLED);
    }
    hold(c, PORT_RECOVERY_MS);
    Ok(r.r32(off) & PORT_LSDA != 0)
}

/// Disable `port`, so a device given back is not left addressed.
pub fn disable<R: Regs>(r: &mut R, port: u8) {
    r.w32(offset(port), PORT_CLEAR_ENABLE);
}
