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

//! Bringing the controller up (firmware let go in setup/handoff.rs): halted,
//! reset, interrupts off, the lists given, run with both schedules on,
//! every port routed here (CONFIGFLAG), the schedules seen on, ports
//! powered. Returns the operational base and the port count.

use crate::env::{hold, until, Clock, Mem, Regs};
use crate::facts::bounds::{HALT_MS, HCRESET_MS, POWER_MS, SCHEDULE_MS};
use crate::facts::port::{PORTS_MAX, PORT_PP, PORT_WC};
use crate::facts::regs::*;
use crate::sched::layout::{qh, ASYNC_HEAD, FRAME_LIST};
use crate::sched::lists::build;

pub const WOULD_NOT_HALT: &str = "controller did not halt";
pub const HCRESET_STUCK: &str = "HCRESET did not clear";
pub const NOT_RUNNING: &str = "the schedules did not start";

pub fn start<R: Regs, C: Clock, M: Mem>(
    r: &mut R,
    c: &C,
    m: &M,
) -> Result<(u32, u8), &'static str> {
    let op = r.r32(CAPLENGTH) & 0xFF;
    let (hcs, hcc) = (r.r32(HCSPARAMS), r.r32(HCCPARAMS));
    let cmd = r.r32(op + USBCMD);
    r.w32(op + USBCMD, cmd & !CMD_RS);
    if !until(c, HALT_MS, || r.r32(op + USBSTS) & STS_HALTED != 0) {
        return Err(WOULD_NOT_HALT);
    }
    r.w32(op + USBCMD, CMD_HCRESET);
    if !until(c, HCRESET_MS, || r.r32(op + USBCMD) & CMD_HCRESET == 0) {
        return Err(HCRESET_STUCK);
    }
    r.w32(op + USBINTR, 0);
    r.w32(op + USBSTS, STS_ALL);
    build(m);
    if hcc & HCC_64BIT != 0 {
        r.w32(op + CTRLDSSEGMENT, 0);
    }
    r.w32(op + PERIODICLISTBASE, m.bus(FRAME_LIST));
    r.w32(op + ASYNCLISTADDR, m.bus(qh(ASYNC_HEAD)));
    r.w32(op + USBCMD, CMD_ITC_8 | CMD_PSE | CMD_ASE | CMD_RS);
    r.w32(op + CONFIGFLAG, CF_EHCI);
    let on = STS_PSS | STS_ASS;
    if !until(c, SCHEDULE_MS, || r.r32(op + USBSTS) & (on | STS_HALTED) == on) {
        return Err(NOT_RUNNING);
    }
    let ports = ((hcs & HCS_N_PORTS) as u8).min(PORTS_MAX);
    if hcs & HCS_PPC != 0 {
        for p in 1..=ports {
            let off = super::port::offset(op, p);
            let raw = r.r32(off);
            r.w32(off, raw & !PORT_WC | PORT_PP);
        }
    }
    // Power good, and a device's connect seen, before anyone asks.
    hold(c, POWER_MS);
    Ok((op, ports))
}
