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

//! OP_POWER_OFF, the kernel's alone (server::medium_rule): FLUSH CACHE EXT,
//! then STANDBY IMMEDIATE unless the machine restarts, each with the budget
//! server::power_off_rule gives it; then the served port is retired, so a
//! request that comes later is answered E_NODEV and nothing spins the disk
//! up again before power goes.

use crate::engine::{flush_within, retire, standby_within};
use crate::log::Line;
use crate::protocol::{Request, E_NODEV, POWER_OFF_FLUSH_ONLY};
use crate::server::error::reply_with_status;
use crate::server::power_off_rule::{power_off, Standby};
use crate::setup::Driver;

pub fn handle(driver: &mut Driver, req: &Request, tx: &mut [u8]) {
    let regs = driver.regs;
    let Some(mut port) = driver.block.take() else {
        return reply_with_status(tx, req, E_NODEV);
    };
    let standby_too = req.flags & POWER_OFF_FLUSH_ONLY == 0;
    let flush = |p: &mut _, ms| flush_within(p, regs, ms);
    let done = power_off(&mut port, standby_too, flush, |p, ms| standby_within(p, regs, ms));
    let status = match done.failure() {
        Ok(()) => 0,
        Err(e) => super::failure::status(&port, e),
    };
    say(done.flushed.is_ok(), done.standby);
    retire(port, regs);
    reply_with_status(tx, req, status);
}

fn say(flushed: bool, standby: Standby) {
    let how: &[u8] = match standby {
        Standby::Done => b"in standby",
        Standby::Failed(_) => b"standby failed",
        Standby::NotAsked => b"left spinning for the restart",
        Standby::NoTime => b"no time left for the standby",
    };
    Line::new()
        .text(b"power-off: ")
        .text(if flushed { b"flushed, " } else { b"flush failed, " })
        .text(how)
        .send();
}
