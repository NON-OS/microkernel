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

//! One request, answered: every frame gets a reply, since its sender waits
//! for one, and every refusal is a named status. The ops and statuses are
//! virtio-gpu's: an op served with a body of the wrong length is E_INVAL,
//! an op not served is E_BAD_OP with no body and E_INVAL with one.

use super::wire::{
    parse, payload, refused, status, Reply, E_BAD_OP, E_INVAL, OP_FLUSH, OP_GET_PRIMARY_SURFACE,
    OP_SET_SCANOUT, OP_TRANSFER_TO_HOST,
};
use super::{flush, primary, scanout, transfer};
use crate::setup::Driver;

/// The reply to `msg` from `sender`, written into `tx`; its length.
pub fn answer(d: &mut Driver, sender: u32, msg: &[u8], tx: &mut Reply) -> usize {
    let Some((req, body)) = parse(msg) else {
        return status(tx, &refused(msg), E_INVAL);
    };
    let code = match req.op {
        OP_GET_PRIMARY_SURFACE if body.is_empty() => match primary::handle(d, sender) {
            Ok(b) => return payload(tx, &req, &b),
            Err(e) => e,
        },
        OP_TRANSFER_TO_HOST => transfer::handle(d, sender, body),
        OP_SET_SCANOUT => scanout::handle(d, sender, body),
        OP_FLUSH => flush::handle(d, sender, body),
        _ if body.is_empty() => E_BAD_OP,
        _ => E_INVAL,
    };
    status(tx, &req, code)
}
