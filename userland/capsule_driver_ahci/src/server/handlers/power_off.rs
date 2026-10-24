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
//! then STANDBY IMMEDIATE (server::power_off_rule), then the served port is
//! retired, so a request that comes later is answered E_NODEV and nothing
//! spins the disk up again before power goes. Each command is bounded by the
//! engine's own command timeout, and the kernel's budget bounds the whole.

use crate::engine::{flush, retire, standby};
use crate::log::Line;
use crate::protocol::{Request, E_NODEV};
use crate::server::error::reply_with_status;
use crate::server::power_off_rule::power_off;
use crate::setup::Driver;

pub fn handle(driver: &mut Driver, req: &Request, tx: &mut [u8]) {
    let regs = driver.regs;
    let Some(mut port) = driver.block.take() else {
        return reply_with_status(tx, req, E_NODEV);
    };
    let done = power_off(&mut port, |p| flush(p, regs), |p| standby(p, regs));
    let status = match done.failure() {
        Ok(()) => 0,
        Err(e) => super::failure::status(&port, e),
    };
    say(done.flushed.is_ok(), done.standby.is_ok());
    retire(port, regs);
    reply_with_status(tx, req, status);
}

fn say(flushed: bool, standby: bool) {
    Line::new()
        .text(b"power-off: ")
        .text(if flushed { b"flushed, " } else { b"flush failed, " })
        .text(if standby { b"in standby" } else { b"standby failed" })
        .send();
}
