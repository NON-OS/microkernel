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

use super::{handlers, respond};
use crate::protocol::{
    Request, E_BAD_OP, E_INVAL, OP_FLUSH, OP_GET_PRIMARY_SURFACE, OP_HEALTHCHECK, OP_SET_SCANOUT,
    OP_TRANSFER_TO_HOST,
};
use crate::setup::Driver;

pub fn dispatch(d: &mut Driver, sender: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    match req.op {
        OP_HEALTHCHECK if body.is_empty() => respond::status(sender, req, 0, tx),
        OP_GET_PRIMARY_SURFACE if body.is_empty() => handlers::primary(d, sender, req, tx),
        OP_TRANSFER_TO_HOST => {
            let r = handlers::transfer(d, sender, body);
            respond::result(sender, req, r, tx)
        }
        OP_SET_SCANOUT => respond::result(sender, req, handlers::scanout(d, sender, body), tx),
        OP_FLUSH => respond::result(sender, req, handlers::flush(d, sender, body), tx),
        // What a GPU does beyond one scanout (resources, 3D) this has not.
        _ if body.is_empty() => respond::status(sender, req, E_BAD_OP, tx),
        _ => respond::status(sender, req, E_INVAL, tx),
    }
}
