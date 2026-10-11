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

//! One checked request routed to its handler. Every op names a unit; one
//! with no disk behind it is answered -6 (ENXIO), the health check among
//! them, which is how a client learns the units.

use nonos_scsi::Hba;

use super::{handlers, reply};
use crate::units::Units;
use crate::wire::{
    Request, E_INVAL, E_NXIO, OP_FLUSH, OP_GEOMETRY, OP_HEALTHCHECK, OP_IDENTIFY, OP_READ, OP_WRITE,
};

pub fn dispatch<H: Hba>(u: &mut Units<H>, req: &Request, body: &[u8], tx: &mut [u8]) {
    if u.disk(req.unit).is_none() {
        return reply::status(u.reply, tx, req, E_NXIO);
    }
    let carries = matches!(req.op, OP_READ | OP_WRITE);
    if !carries && !body.is_empty() {
        return reply::status(u.reply, tx, req, E_INVAL);
    }
    match req.op {
        OP_HEALTHCHECK => reply::status(u.reply, tx, req, 0),
        OP_GEOMETRY => handlers::geometry(u, req, tx),
        OP_IDENTIFY => handlers::identify(u, req, tx),
        OP_READ => handlers::read(u, req, body, tx),
        OP_WRITE => handlers::write(u, req, body, tx),
        OP_FLUSH => handlers::flush(u, req, tx),
        _ => reply::status(u.reply, tx, req, E_INVAL),
    }
}
