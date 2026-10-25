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
//! OP_SLOT_OF_PORT's answer: the shared slot and its claimed interfaces.

use crate::protocol::{encode_response_header, write_status, Request, RESP_HDR_LEN, STATUS_LEN};
use crate::server::reply::send;

pub(super) fn slot_of_port(tx: &mut [u8], req: &Request, slot: u8, held: u32) {
    let plen = (STATUS_LEN + 8) as u32;
    encode_response_header(tx, req, plen);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let o = RESP_HDR_LEN + STATUS_LEN;
    tx[o..o + 4].copy_from_slice(&[slot, 0, 0, 0]);
    tx[o + 4..o + 8].copy_from_slice(&held.to_le_bytes());
    send(tx.as_ptr(), RESP_HDR_LEN + plen as usize);
}
