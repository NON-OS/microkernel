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

//! READ and WRITE on a unit.

use super::rw_parse::parse;
use crate::platform::Opened;
use crate::protocol::{Request, E_MSGSIZE, RESP_HDR_LEN, RW_HEADER_LEN, STATUS_LEN};
use crate::server::reply;
use crate::ufs::disk::{rw, Unit};
use crate::ufs::error::wire_status;

pub fn serve_read(o: &Opened, u: &Unit, req: &Request, body: &[u8], tx: &mut [u8]) {
    let (lba, n) = match parse(body, u.blocks, o.ufs.max_blocks(u)) {
        Ok(v) => v,
        Err(s) => return reply::status(tx, req, s),
    };
    let bytes = n as usize * u.block_len as usize;
    let at = RESP_HDR_LEN + STATUS_LEN;
    if let Err(e) = rw::read(&o.env, &o.ufs, u, lba, n, &mut tx[at..at + bytes]) {
        return reply::status(tx, req, wire_status(e));
    }
    reply::finish(tx, req, bytes);
}

pub fn serve_write(o: &Opened, u: &Unit, req: &Request, body: &[u8], tx: &mut [u8]) {
    let (lba, n) = match parse(body, u.blocks, o.ufs.max_blocks(u)) {
        Ok(v) => v,
        Err(s) => return reply::status(tx, req, s),
    };
    if body.len() != RW_HEADER_LEN + n as usize * u.block_len as usize {
        return reply::status(tx, req, E_MSGSIZE);
    }
    let s = rw::write(&o.env, &o.ufs, u, lba, n, &body[RW_HEADER_LEN..])
        .map_or_else(wire_status, |()| 0);
    reply::status(tx, req, s);
}
