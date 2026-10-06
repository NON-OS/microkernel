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

//! READ_BLOCKS and WRITE_BLOCKS.

use crate::emmc::{wire_status, Opened, SECTOR_SIZE};
use crate::protocol::{Request, E_MSGSIZE, READ_REQ_LEN, RESP_HDR_LEN, RW_HEADER_LEN, STATUS_LEN};
use crate::server::error::reply_with_status;

use super::reply::finish;

pub(super) fn read(o: &mut Opened, req: &Request, body: &[u8], tx: &mut [u8]) {
    if req.payload_len as usize != READ_REQ_LEN {
        return reply_with_status(tx, req, E_MSGSIZE);
    }
    let (lba, n) = match super::super::rw_parse::parse(body, o.disk.capacity_sectors()) {
        Ok(v) => v,
        Err(s) => return reply_with_status(tx, req, s),
    };
    let bytes = n as usize * SECTOR_SIZE;
    let at = RESP_HDR_LEN + STATUS_LEN;
    if let Err(e) = o.disk.read(lba, n, &mut tx[at..at + bytes]) {
        return reply_with_status(tx, req, wire_status(e));
    }
    finish(tx, req, bytes);
}

pub(super) fn write(o: &mut Opened, req: &Request, body: &[u8], tx: &mut [u8]) {
    let (lba, n) = match super::super::rw_parse::parse(body, o.disk.capacity_sectors()) {
        Ok(v) => v,
        Err(s) => return reply_with_status(tx, req, s),
    };
    let bytes = n as usize * SECTOR_SIZE;
    if body.len() != RW_HEADER_LEN + bytes || req.payload_len as usize != RW_HEADER_LEN + bytes {
        return reply_with_status(tx, req, E_MSGSIZE);
    }
    let status = match o.disk.write(lba, n, &body[RW_HEADER_LEN..]) {
        Ok(()) => 0,
        Err(e) => wire_status(e),
    };
    reply_with_status(tx, req, status);
}
