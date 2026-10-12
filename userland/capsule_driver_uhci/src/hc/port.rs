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

//! A root port: reset, enabled, and its speed. PORTSC's change bits clear
//! when written as 1, so every write here carries them as 0 unless it
//! means to clear them.

use crate::env::{hold, until, Clock, Io};
use crate::facts::bounds::{PORT_ENABLE_MS, PORT_RECOVERY_MS, PORT_RESET_MS};
use crate::facts::io::*;

pub const NO_DEVICE: &str = "nothing connected on the port";
pub const NOT_ENABLED: &str = "port did not enable after reset";

pub fn offset(port: u8) -> u16 {
    PORTSC1 + 2 * (port as u16 - 1)
}

pub fn valid(port: u8) -> bool {
    (1..=ROOT_PORTS).contains(&port)
}

/// Reset `port` and enable it; true for a low-speed device.
pub fn reset<I: Io, C: Clock>(io: &mut I, c: &C, port: u8) -> Result<bool, &'static str> {
    let off = offset(port);
    if io.r16(off) & PORT_CCS == 0 {
        return Err(NO_DEVICE);
    }
    let keep = |v: u16| v & !PORT_WC;
    let v = io.r16(off);
    io.w16(off, keep(v) | PORT_PR);
    hold(c, PORT_RESET_MS);
    let v = io.r16(off);
    io.w16(off, keep(v) & !PORT_PR);
    hold(c, 1);
    let v = io.r16(off);
    // Enabled, and the changes the reset raised cleared.
    io.w16(off, keep(v) | PORT_PE | PORT_WC);
    if !until(c, PORT_ENABLE_MS, || io.r16(off) & PORT_PE != 0) {
        return Err(NOT_ENABLED);
    }
    hold(c, PORT_RECOVERY_MS);
    Ok(io.r16(off) & PORT_LSDA != 0)
}

/// Disable `port`, so a device given back is not left addressed.
pub fn disable<I: Io>(io: &mut I, port: u8) {
    let off = offset(port);
    let v = io.r16(off);
    io.w16(off, v & !(PORT_WC | PORT_PE));
}
