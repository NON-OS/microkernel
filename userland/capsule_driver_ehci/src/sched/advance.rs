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
//! The async advance doorbell: once it is answered the controller holds
//! no copy of a QH taken from the ring. An answer that comes late, or a
//! doorbell bit cleared with no answer, counts as Linux's IAA watchdog
//! counts it (ehci-timer.c), and a halted controller caches nothing. Past
//! that the controller is taken as dead, as Linux takes one that fails:
//! run, schedules and doorbell cleared, then seen to halt. One that will
//! not halt still holds what it held, so the QH may not be touched: a
//! controller still caching a rewritten QH writes its stale copy back.

use crate::env::{until, Clock, Regs};
use crate::facts::bounds::{DOORBELL_MS, HALT_MS};
use crate::facts::regs::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Advance {
    /// The controller let the QH go.
    Seen,
    /// It never answered and was stopped: the QH is free, the host dead.
    Stopped,
    /// It never answered and would not stop: the QH stays as it was.
    Wedged,
}

/// The controller would not let a QH go: it is stopped, or wedged.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Dead;

/// Ring the doorbell and judge the answer.
pub fn doorbell<R: Regs, C: Clock>(r: &mut R, c: &C, op: u32) -> Advance {
    r.w32(op + USBSTS, STS_IAA);
    let cmd = r.r32(op + USBCMD);
    r.w32(op + USBCMD, cmd | CMD_IAAD);
    let rang = until(c, DOORBELL_MS, || {
        let sts = r.r32(op + USBSTS);
        sts != !0 && sts & STS_IAA != 0
    });
    let (cmd, sts) = (r.r32(op + USBCMD), r.r32(op + USBSTS));
    r.w32(op + USBSTS, STS_IAA);
    let late = sts & (STS_IAA | STS_HALTED) != 0 || cmd & CMD_IAAD == 0;
    match rang || (sts != !0 && cmd != !0 && late) {
        true => Advance::Seen,
        false if stop(r, c, op) => Advance::Stopped,
        false => Advance::Wedged,
    }
}

/// The controller stopped, as Linux stops one it gives up on: true once
/// it reads halted (a controller gone reads all ones and is not).
pub(super) fn stop<R: Regs, C: Clock>(r: &mut R, c: &C, op: u32) -> bool {
    let cmd = r.r32(op + USBCMD);
    r.w32(op + USBCMD, cmd & !(CMD_RS | CMD_ASE | CMD_PSE | CMD_IAAD));
    until(c, HALT_MS, || {
        let sts = r.r32(op + USBSTS);
        sts != !0 && sts & STS_HALTED != 0
    })
}
