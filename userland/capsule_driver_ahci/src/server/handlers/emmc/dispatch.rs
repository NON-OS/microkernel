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

//! Routing one request to the op that answers it.

use crate::emmc::{wire_status, Opened};
use crate::protocol::{
    Request, CAPACITY_PAYLOAD_LEN, CONTROLLER_INFO_PAYLOAD_LEN, E_INVAL, OP_CAPACITY,
    OP_CONTROLLER_INFO, OP_FLUSH, OP_HEALTHCHECK, OP_IDENTIFY, OP_PORT_LIST, OP_READ_BLOCKS,
    OP_WRITE_BLOCKS,
};
use crate::server::error::reply_with_status;

use super::describe::{identify, port_list};
use super::reply::body_reply;
use super::rw::{read, write};

pub fn dispatch(o: &mut Opened, req: &Request, body: &[u8], tx: &mut [u8]) {
    match req.op {
        OP_HEALTHCHECK => super::super::health::handle(req, tx),
        OP_CONTROLLER_INFO => {
            let info = o.disk.controller_info();
            body_reply(tx, req, &info[..CONTROLLER_INFO_PAYLOAD_LEN.min(info.len())]);
        }
        OP_PORT_LIST => port_list(o, req, tx),
        OP_CAPACITY => {
            let cap: [u8; CAPACITY_PAYLOAD_LEN] = o.disk.capacity_sectors().to_le_bytes();
            body_reply(tx, req, &cap);
        }
        OP_READ_BLOCKS => read(o, req, body, tx),
        OP_WRITE_BLOCKS => write(o, req, body, tx),
        OP_FLUSH => {
            let status = match o.disk.flush() {
                Ok(()) => 0,
                Err(e) => wire_status(e),
            };
            reply_with_status(tx, req, status);
        }
        OP_IDENTIFY => identify(o, req, tx),
        _ => reply_with_status(tx, req, E_INVAL),
    }
}
