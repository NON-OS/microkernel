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

//! READ and WRITE, to the card in the slot.

use crate::emmc::{wire_status, Opened, SECTOR_SIZE};
use crate::protocol::{Request, E_MSGSIZE, E_ROFS, RESP_HDR_LEN, RW_HEADER_LEN, STATUS_LEN};
use crate::server::error::reply_with_status;
use crate::server::reply::finish;

use super::rw_parse::parse;

pub fn serve_read(o: &mut Opened, req: &Request, body: &[u8], tx: &mut [u8]) {
    let max = o.slot.max_sectors();
    let disk = match o.slot.data_disk() {
        Ok(d) => d,
        Err(e) => return reply_with_status(tx, req, wire_status(e)),
    };
    let (lba, n) = match parse(body, disk.capacity_sectors(), max) {
        Ok(v) => v,
        Err(s) => return reply_with_status(tx, req, s),
    };
    let bytes = n as usize * SECTOR_SIZE;
    let at = RESP_HDR_LEN + STATUS_LEN;
    if let Err(e) = disk.read(lba, n, &mut tx[at..at + bytes]) {
        return reply_with_status(tx, req, wire_status(e));
    }
    finish(tx, req, bytes);
}

pub fn serve_write(o: &mut Opened, req: &Request, body: &[u8], tx: &mut [u8]) {
    let max = o.slot.max_sectors();
    let writable = o.slot.writable();
    let disk = match o.slot.data_disk() {
        Ok(d) => d,
        Err(e) => return reply_with_status(tx, req, wire_status(e)),
    };
    if !writable {
        return reply_with_status(tx, req, E_ROFS);
    }
    let (lba, n) = match parse(body, disk.capacity_sectors(), max) {
        Ok(v) => v,
        Err(s) => return reply_with_status(tx, req, s),
    };
    if body.len() != RW_HEADER_LEN + n as usize * SECTOR_SIZE {
        return reply_with_status(tx, req, E_MSGSIZE);
    }
    let status = disk.write(lba, n, &body[RW_HEADER_LEN..]).map_or_else(wire_status, |()| 0);
    reply_with_status(tx, req, status);
}
