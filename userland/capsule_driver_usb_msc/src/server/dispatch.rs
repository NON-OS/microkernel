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

//! Each request to its handler, once its sender is one this driver serves
//! (`admits`). The codec ops work on a state of their own: one of them
//! resets the residue quirk of whatever state it is given.

use super::admits::admits;
use super::serve::Server;
use crate::protocol::*;
use crate::server::{handlers, respond};

pub(super) fn dispatch(server: &mut Server, sender: u32, req: Request, body: &[u8], tx: &mut [u8]) {
    if !admits(req.op, sender) {
        let _ = respond::status(sender, &req, E_ACCES, tx);
        return;
    }
    let Server { state, codec, scanner } = server;
    match req.op {
        OP_BLK_CAPACITY | OP_BLK_READ | OP_BLK_WRITE | OP_BLK_FLUSH => {
            handlers::block::handle(state, &mut scanner.medium, sender, &req, body, tx)
        }
        OP_HEALTHCHECK if body.is_empty() => handlers::health::handle(sender, &req, tx),
        OP_PROBE_CONFIG => handlers::probe_config::handle(codec, sender, &req, body, tx),
        OP_BUILD_INQUIRY if body.is_empty() => {
            handlers::build_inquiry::handle(codec, sender, &req, tx)
        }
        OP_BUILD_READ_CAPACITY10 if body.is_empty() => {
            handlers::build_capacity::handle(codec, sender, &req, tx)
        }
        OP_BUILD_READ10 => handlers::build_read::handle(codec, sender, &req, body, tx),
        OP_BUILD_WRITE10 => handlers::build_write::handle(codec, sender, &req, body, tx),
        OP_ACCEPT_CSW => handlers::accept_csw::handle(codec, sender, &req, body, tx),
        OP_BUILD_TEST_UNIT_READY if body.is_empty() => {
            handlers::build_tur::handle(codec, sender, &req, tx)
        }
        OP_BUILD_REQUEST_SENSE => {
            handlers::build_request_sense::handle(codec, sender, &req, body, tx)
        }
        OP_DECODE_INQUIRY => handlers::decode_inquiry::handle(sender, &req, body, tx),
        OP_DECODE_CAPACITY => handlers::decode_capacity::handle(sender, &req, body, tx),
        OP_DECODE_SENSE => handlers::decode_sense::handle(sender, &req, body, tx),
        OP_GET_STATE if body.is_empty() => {
            handlers::get_state::handle(state, scanner, sender, &req, tx)
        }
        _ if body.is_empty() => {
            let _ = respond::status(sender, &req, E_BAD_OP, tx);
        }
        _ => {
            let _ = respond::status(sender, &req, E_INVAL, tx);
        }
    }
}
