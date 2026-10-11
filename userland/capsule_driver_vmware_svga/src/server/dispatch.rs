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

//! Which handler a request goes to. Only the owner of the primary surface
//! may draw; anyone may ask whether the driver is healthy.

use super::handlers;
use crate::protocol::{
    Request, E_BAD_OP, E_BUSY, OP_FLUSH, OP_GET_PRIMARY_SURFACE, OP_HEALTHCHECK, OP_SET_SCANOUT,
    OP_TRANSFER_TO_HOST,
};
use crate::setup::Driver;

/// A handler that replies with a body does so itself and answers None.
pub fn dispatch(
    d: &mut Driver,
    sender: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Option<i32> {
    if req.op == OP_GET_PRIMARY_SURFACE {
        return handlers::primary(d, sender, req, tx);
    }
    if req.op == OP_HEALTHCHECK {
        return Some(handlers::health(d));
    }
    if d.owner != sender {
        return Some(E_BUSY);
    }
    Some(match req.op {
        OP_TRANSFER_TO_HOST => handlers::transfer(d, body),
        OP_SET_SCANOUT => handlers::scanout(d, body),
        OP_FLUSH => handlers::flush(d, body),
        _ => E_BAD_OP,
    })
}
