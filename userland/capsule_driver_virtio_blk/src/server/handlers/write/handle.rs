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

use super::request::write_request;
use crate::io::submit;
use crate::protocol::{Request, E_IO, RW_HEADER_LEN};
use crate::queue::Direction;
use crate::server::error::reply_with_status;
use crate::server::handlers::status::status_of;
use crate::setup::Driver;

pub fn handle(driver: &mut Driver, req: &Request, body: &[u8], tx: &mut [u8]) {
    let parsed = match write_request(driver.capacity_sectors, req, body) {
        Ok(v) => v,
        Err(status) => {
            reply_with_status(tx, req, status);
            return;
        }
    };
    // The queue hands its buffer over only while no request holds it.
    let data = body.get(RW_HEADER_LEN..RW_HEADER_LEN + parsed.bytes_n);
    if !data.is_some_and(|d| driver.queue.fill(d)) {
        reply_with_status(tx, req, E_IO);
        return;
    }
    let outcome = submit(
        driver.transport,
        &mut driver.queue,
        driver.irq_grant,
        Direction::Write,
        parsed.lba,
        parsed.nsectors,
    );
    reply_with_status(tx, req, status_of(outcome));
}
