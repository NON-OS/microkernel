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

//! OP_POWER_OFF, the kernel's alone (server::medium_rule): before power goes,
//! a flush, so the volatile write cache is on the medium; then the normal
//! shutdown and the reset (setup::power_off). The I/O queue is dropped after
//! either way, so a request that comes later is answered E_NODEV and nothing
//! reaches a controller that was told power is going. No recovery runs: a
//! flush that stalls is given up, and the shutdown still goes.

use crate::error::reason;
use crate::log::{emit, Line};
use crate::protocol::Request;
use crate::server::error::reply_with_status;
use crate::server::power_off_rule::{power_off, PoweredOff};
use crate::setup::Driver;

pub fn handle(driver: &mut Driver, req: &Request, tx: &mut [u8]) {
    let regs = driver.device.regs;
    let rtd3e_us = driver.identity.rtd3e_us;
    let Driver { device, io, .. } = driver;
    let flush = || match io.as_mut() {
        Some(q) => q.retried(0, |q| q.flush(regs)).map_err(|e| q.failure_status(e)),
        None => Ok(()),
    };
    let done = power_off(flush, || device.power_off(rtd3e_us));
    *io = None;
    say(&done);
    reply_with_status(tx, req, done.status());
}

fn say(done: &PoweredOff) {
    let mut line = Line::new();
    line.text(b"power-off: ");
    match done.flushed {
        Ok(()) => line.text(b"flushed, "),
        Err(status) => {
            line.text(b"flush failed with -").dec(status.unsigned_abs() as u64).text(b", ")
        }
    };
    match done.shut {
        Ok(()) => line.text(b"shut down"),
        Err(e) => line.text(reason(e).as_bytes()),
    };
    emit(&mut line);
}
