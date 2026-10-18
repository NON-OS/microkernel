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
    serves_unit, Request, E_INVAL, E_NXIO, OP_FLUSH, OP_GEOMETRY, OP_HEALTHCHECK, OP_IDENTIFY,
    OP_READ, OP_WRITE,
};

use super::error::reply_with_status;
use super::handlers::{geometry, identify, rw};

pub fn dispatch(o: &mut Opened, req: &Request, body: &[u8], tx: &mut [u8]) {
    if !serves_unit(req.unit) {
        return reply_with_status(tx, req, E_NXIO);
    }
    let bare = matches!(req.op, OP_HEALTHCHECK | OP_GEOMETRY | OP_FLUSH | OP_IDENTIFY);
    if bare && !body.is_empty() {
        return reply_with_status(tx, req, E_INVAL);
    }
    match req.op {
        OP_HEALTHCHECK => reply_with_status(tx, req, 0),
        OP_GEOMETRY => geometry::serve(o, req, tx),
        OP_IDENTIFY => identify::serve(o, req, tx),
        OP_READ => rw::serve_read(o, req, body, tx),
        OP_WRITE => rw::serve_write(o, req, body, tx),
        OP_FLUSH => {
            let status = match o.slot.data_disk() {
                Ok(d) => d.flush().map_or_else(wire_status, |()| 0),
                Err(e) => wire_status(e),
            };
            reply_with_status(tx, req, status);
        }
        _ => reply_with_status(tx, req, E_INVAL),
    }
}
