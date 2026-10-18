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

//! Bringing the controller up from whatever firmware left: stopped and
//! seen halted, a global reset held, then a host reset seen to clear; every
//! status cleared, no interrupt enabled (the driver polls), the schedule
//! built and its frame list given, then run with the configure flag set.

use crate::env::{hold, until, Clock, Io, Mem};
use crate::facts::bounds::{GRESET_MS, HALT_MS, HCRESET_MS};
use crate::facts::io::*;
use crate::sched::layout::FRAME_LIST;
use crate::sched::skeleton::build;

pub const HCRESET_STUCK: &str = "HCRESET did not clear in 50 ms";
pub const WOULD_NOT_RUN: &str = "controller halted after RS was set";

pub fn start<I: Io, C: Clock, M: Mem>(io: &mut I, c: &C, m: &M) -> Result<(), &'static str> {
    io.w16(USBCMD, 0);
    // A controller that will not halt is reset anyway: the global reset
    // stops it whatever it was doing.
    let _ = until(c, HALT_MS, || io.r16(USBSTS) & STS_HCH != 0);
    io.w16(USBCMD, CMD_GRESET);
    hold(c, GRESET_MS);
    io.w16(USBCMD, 0);
    io.w16(USBCMD, CMD_HCRESET);
    if !until(c, HCRESET_MS, || io.r16(USBCMD) & CMD_HCRESET == 0) {
        return Err(HCRESET_STUCK);
    }
    io.w16(USBINTR, 0);
    io.w16(USBSTS, STS_ALL);
    build(m);
    io.w32(FLBASEADD, m.bus(FRAME_LIST));
    io.w16(FRNUM, 0);
    io.w16(USBCMD, CMD_RS | CMD_CF | CMD_MAXP);
    hold(c, 2);
    if io.r16(USBSTS) & STS_HCH != 0 {
        return Err(WOULD_NOT_RUN);
    }
    Ok(())
}
