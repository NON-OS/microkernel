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

//! `OP_RX_PACKET`. Hands up the next frame the return ring holds, `E_AGAIN`
//! when the status block shows none, or `E_IO` for one the part returned
//! with an error or a length this driver does not take. Every buffer goes
//! back on the standard ring once its bytes are copied out, and both
//! mailboxes are written before the reply (tg3_rx).

use crate::protocol::{
    encode_response_header, write_status, Request, E_AGAIN, E_IO, RESP_HDR_LEN,
    RX_PAYLOAD_PREFIX_LEN, STATUS_LEN,
};
use crate::queue::ret::Rx;
use crate::server::error::{reply, reply_with_status};
use crate::setup::Driver;

use super::rx_return::{give_back, next};

/*
 * SAFETY: `buffer_va(buf)` is a slot of the RX buffer grant, RX_BUFFER_LEN
 * bytes, longer than any length `consume` reports.
 */
pub fn handle(sender: u32, d: &mut Driver, req: &Request, tx: &mut [u8]) {
    let (buf, len) = match next(d) {
        Some(Rx::Frame { buf, len }) => (buf, len as usize),
        other => {
            let dropped = match other {
                Some(Rx::Dropped { buf }) => Some(buf),
                _ => None,
            };
            give_back(d, dropped);
            let status = if dropped.is_some() { E_IO } else { E_AGAIN };
            return reply_with_status(sender, tx, req, status);
        }
    };
    let body_len = RX_PAYLOAD_PREFIX_LEN + len;
    encode_response_header(tx, req, (STATUS_LEN + body_len) as u32);
    write_status(&mut tx[RESP_HDR_LEN..], 0);
    let prefix_off = RESP_HDR_LEN + STATUS_LEN;
    let body_off = prefix_off + RX_PAYLOAD_PREFIX_LEN;
    tx[prefix_off..body_off].copy_from_slice(&(len as u32).to_le_bytes());
    // Bounded by the reply buffer too, so a drift among the size constants
    // can never let a part-written length overflow it.
    let n = len.min(tx.len().saturating_sub(body_off));
    let src = d.rx.buffer_va(buf) as *const u8;
    unsafe { core::ptr::copy_nonoverlapping(src, tx[body_off..].as_mut_ptr(), n) };
    give_back(d, Some(buf));
    reply(sender, tx, prefix_off + body_len);
}
