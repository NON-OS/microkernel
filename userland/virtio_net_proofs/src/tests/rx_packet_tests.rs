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

//! The OP_RX_PACKET handler and the buffer it answers from: the frame goes
//! out and does not stay, and a buffer with no room for a frame takes none.

use super::drive::{driver, status};
use super::harness::Dev;
use crate::constants::VIRTIO_NET_HDR_LEN;
use crate::protocol::{Request, E_IO, OP_RX_PACKET, REPLY_MAX, RX_FRAME_AT};
use crate::server::error::take_sent;
use crate::server::handlers::rx_packet;

const HDR: u32 = VIRTIO_NET_HDR_LEN as u32;
const REQ: Request = Request { op: OP_RX_PACKET, flags: 0, request_id: 1, payload_len: 0 };

#[test]
fn the_frame_is_wiped_from_the_reply_buffer_once_sent() {
    let (rx, tx) = (Dev::rx(), Dev::tx());
    let mut d = driver(&rx, &tx);
    rx.bufs.put(VIRTIO_NET_HDR_LEN, &[0xC3; 600]);
    rx.complete(0, 0, HDR + 600);
    let mut buf = vec![0u8; REPLY_MAX];
    let _ = take_sent();
    rx_packet::handle(1, &mut d, &REQ, &mut buf);
    let sent = take_sent().remove(0).unwrap();
    assert!(sent[RX_FRAME_AT..].iter().all(|&b| b == 0xC3), "the frame was sent");
    assert!(buf[RX_FRAME_AT..].iter().all(|&b| b == 0), "the frame stayed in the buffer");
}

#[test]
fn a_reply_buffer_too_short_for_a_frame_answers_an_error() {
    let (rx, tx) = (Dev::rx(), Dev::tx());
    let mut d = driver(&rx, &tx);
    rx.complete(0, 0, HDR + 60);
    let _ = take_sent();
    rx_packet::handle(1, &mut d, &REQ, &mut vec![0u8; REPLY_MAX - 1]);
    assert_eq!(status(&take_sent().remove(0).unwrap()), E_IO);
    assert_eq!(d.rx.last_used, 0, "no frame was taken off the ring");
}
