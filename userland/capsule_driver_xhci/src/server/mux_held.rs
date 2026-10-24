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

//! OP_USB_HELD's reply: the devices held for the owner's answer, each its
//! slot and the ticket an answer must carry.

use crate::protocol::{encode_response_header, write_status, Request, RESP_HDR_LEN, STATUS_LEN};
use crate::server::reply::send;

/// Held devices listed in one reply, at most.
const LISTED: usize = 32;

pub(super) fn held(list: impl Iterator<Item = (u8, u32)>, req: &Request, tx: &mut [u8]) -> bool {
    let at = RESP_HDR_LEN + STATUS_LEN;
    let mut n = 0usize;
    for (slot, ticket) in list.take(LISTED) {
        tx[at + 1 + 5 * n] = slot;
        tx[at + 2 + 5 * n..at + 6 + 5 * n].copy_from_slice(&ticket.to_le_bytes());
        n += 1;
    }
    tx[at] = n as u8;
    encode_response_header(tx, req, (STATUS_LEN + 1 + 5 * n) as u32);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    send(tx.as_ptr(), at + 1 + 5 * n);
    true
}
