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

//! A driver over a pair of fake queues and the model register file, and
//! requests to it the way net.core and net.l2 make them.

use super::harness::Dev;
use crate::constants::NET_CFG_STATUS;
use crate::protocol::{Request, HDR_LEN, REPLY_MAX, RESP_HDR_LEN};
use crate::regs::{fresh, with, Regs};
use crate::rx::take_one;
use crate::server::dispatch::serve;
use crate::server::error::take_sent;
use crate::setup::Driver;
use crate::transport::Transport;

/// A legacy driver over `rx` and `tx`, link status reported and up.
pub fn driver(rx: &Dev, tx: &Dev) -> Driver {
    fresh(256);
    with(|m| m.link = 1);
    Driver {
        device_id: 1,
        rx: rx.rx_queue(),
        tx: tx.tx_queue(),
        transport: Transport::Legacy(Regs),
        mac: [2, 0, 0, 0, 0, 1],
        status_supported: true,
        net_status_offset: NET_CFG_STATUS,
    }
}

/// Serve one request of `op` carrying `body` through the shipping dispatch:
/// whether the device was found stalled, and the reply (`None` when the
/// length it asked to send ran past its buffer).
pub fn call(d: &mut Driver, op: u16, body: &[u8]) -> (bool, Option<Vec<u8>>) {
    let req = Request { op, flags: 0, request_id: 9, payload_len: body.len() as u32 };
    let mut msg = vec![0u8; HDR_LEN];
    crate::protocol::encode_response_header(&mut msg, &req, req.payload_len);
    msg.extend_from_slice(body);
    let mut tx = vec![0u8; REPLY_MAX];
    let _ = take_sent();
    let stalled = serve(1, d, &msg, &mut None, &mut tx);
    let mut sent = take_sent();
    assert_eq!(sent.len(), 1, "one reply per request");
    (stalled, sent.remove(0))
}

/// The status word of a reply.
pub fn status(reply: &[u8]) -> i32 {
    i32::from_le_bytes(reply[RESP_HDR_LEN..RESP_HDR_LEN + 4].try_into().unwrap())
}

/// Report one entry for buffer `slot` and take it: the frame's length and
/// the buffer posted back.
pub fn take(dev: &Dev, hdr: usize, slot: u32, used_len: u32, out: &mut [u8]) -> (usize, u16) {
    let mut rx = dev.rx_queue().with_hdr_len(hdr);
    rx.next_avail = 7;
    dev.complete(0, slot, used_len);
    let got = take_one(&mut rx, out).expect("a pending entry");
    assert_eq!(rx.next_avail, 8, "used_len {used_len}: the buffer is not posted back");
    (got, dev.avail(7))
}
