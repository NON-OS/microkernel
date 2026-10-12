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

//! A root port: reset (held, then ended and seen to end), its enable read
//! back; handed to the companion; or disabled. Writes carry the change
//! bits as 0 unless they mean to clear them.

use crate::env::{hold, until, Clock, Regs};
use crate::facts::bounds::{PORT_RECOVERY_MS, PORT_RESET_END_MS, PORT_RESET_MS};
use crate::facts::port::*;

pub const RESET_STUCK: &str = "port reset did not end";

pub fn offset(op: u32, port: u8) -> u32 {
    op + PORTSC + 4 * (port as u32 - 1)
}

/// Reset the port at `off`; true when it then is enabled, which only a
/// high-speed device's port is.
pub fn reset<R: Regs, C: Clock>(r: &mut R, c: &C, off: u32) -> Result<bool, &'static str> {
    let raw = r.r32(off);
    r.w32(off, raw & !(PORT_WC | PORT_PE) | PORT_PR);
    hold(c, PORT_RESET_MS);
    let raw = r.r32(off);
    r.w32(off, raw & !(PORT_WC | PORT_PR));
    if !until(c, PORT_RESET_END_MS, || r.r32(off) & PORT_PR == 0) {
        return Err(RESET_STUCK);
    }
    hold(c, PORT_RECOVERY_MS);
    Ok(r.r32(off) & PORT_PE != 0)
}

/// Give the port to the companion controller.
pub fn hand<R: Regs>(r: &mut R, off: u32) {
    let raw = r.r32(off);
    r.w32(off, raw & !PORT_WC | PORT_OWNER);
}

pub fn disable<R: Regs>(r: &mut R, off: u32) {
    let raw = r.r32(off);
    r.w32(off, raw & !(PORT_WC | PORT_PE));
}
