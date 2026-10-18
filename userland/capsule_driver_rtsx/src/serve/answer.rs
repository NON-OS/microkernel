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

//! What a decoded request gets. The body is held to the length the header
//! claims and the op allows; the sender is admitted or refused (admits.rs);
//! then, for an op that reaches the card, the slot is looked at first, so a
//! card swapped or taken out since the last look is caught before this
//! request touches it. A card that fails the request is checked again
//! before it serves another (fault.rs).

use super::admits::permits;
use super::blocks::{read, write};
use super::fault::Fault;
use super::identity::{capacity, identify};
use super::medium::Medium;
use super::slot::look;
use crate::card::wait_ready;
use crate::protocol::*;
use crate::setup::Driver;

pub(super) fn answer(
    drv: &mut Driver,
    medium: &mut Medium,
    sender: u32,
    req: &Request,
    body: &[u8],
    out: &mut [u8],
) -> Result<usize, Fault> {
    if body.len() != req.payload_len as usize {
        return Err(Fault::Refused(E_MSGSIZE));
    }
    let bare = matches!(req.op, OP_HEALTHCHECK | OP_CAPACITY | OP_FLUSH | OP_IDENTIFY);
    if bare && !body.is_empty() {
        return Err(Fault::Refused(E_INVAL));
    }
    if !permits(req.op, sender) {
        return Err(Fault::Refused(E_ACCES));
    }
    if req.op == OP_HEALTHCHECK {
        return Ok(0);
    }
    look(drv, medium);
    let Medium::Ready(card) = *medium else { return Err(Fault::Refused(medium.refusal())) };
    let done = match req.op {
        OP_CAPACITY => capacity(&card, out),
        OP_IDENTIFY => identify(&card, sender, out),
        OP_READ => read(drv, &card, body, out),
        OP_WRITE => write(drv, &card, body),
        OP_FLUSH => wait_ready(drv, &card).map(|_| 0).map_err(Fault::Card),
        _ => Err(Fault::Refused(E_INVAL)),
    };
    if done.is_err_and(Fault::unsettles) {
        *medium = Medium::Check(card);
    }
    done
}
