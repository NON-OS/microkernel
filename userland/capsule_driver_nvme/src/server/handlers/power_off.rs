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

//! OP_POWER_OFF, the kernel's alone (server::medium_rule): the normal
//! shutdown (setup::power_off), with no flush ahead of it and no reset after
//! (server::power_off_rule says why). The I/O queue is dropped after either
//! way, so a request that comes later is answered E_NODEV and nothing
//! reaches a controller that was told power is going.

use crate::error::reason;
use crate::log::{emit, Line};
use crate::protocol::Request;
use crate::server::error::reply_with_status;
use crate::server::power_off_rule::status;
use crate::setup::Driver;

pub fn handle(driver: &mut Driver, req: &Request, tx: &mut [u8]) {
    let shut = driver.device.power_off(driver.identity.rtd3e_us);
    driver.io = None;
    let mut line = Line::new();
    line.text(b"power-off: ");
    match shut {
        Ok(()) => line.text(b"shut down"),
        Err(e) => line.text(reason(e).as_bytes()),
    };
    emit(&mut line);
    reply_with_status(tx, req, status(shut));
}
