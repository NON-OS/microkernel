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

//! OP_RX_PACKET through the shipping dispatch, against a device that reports
//! frames longer than Ethernet allows (a jumbo bridge, a hostile length).
//! The reply always fits its buffer; the capsule never panics.

use super::drive::{call, driver, status};
use super::harness::Dev;
use crate::constants::{MAX_ETHERNET_FRAME, VIRTIO_NET_HDR_LEN};
use crate::protocol::{E_AGAIN, OP_RX_PACKET, REPLY_MAX, RX_FRAME_AT};

const HDR: u32 = VIRTIO_NET_HDR_LEN as u32;

/// The frame length an OP_RX_PACKET reply carries.
fn frame_len(reply: &[u8]) -> usize {
    u32::from_le_bytes(reply[RX_FRAME_AT - 4..RX_FRAME_AT].try_into().unwrap()) as usize
}

#[test]
fn any_used_length_gets_a_reply_that_fits_its_buffer() {
    let (rx, tx) = (Dev::rx(), Dev::tx());
    let mut d = driver(&rx, &tx);
    let lens = (0..=4096u32).chain([HDR + 1525, HDR + 2038, u32::MAX]);
    for (n, used_len) in lens.enumerate() {
        rx.complete(n as u16, (n % 64) as u32, used_len);
        let (_, reply) = call(&mut d, OP_RX_PACKET, &[]);
        let reply = reply.expect("the reply fits its buffer");
        assert!(reply.len() <= REPLY_MAX);
        assert_eq!(status(&reply), 0);
        let want = used_len.saturating_sub(HDR) as usize;
        let want = if want <= MAX_ETHERNET_FRAME { want } else { 0 };
        assert_eq!(frame_len(&reply), want, "used_len {used_len}");
        assert_eq!(reply.len(), RX_FRAME_AT + want);
    }
}

#[test]
fn an_idle_ring_answers_again_later() {
    let (rx, tx) = (Dev::rx(), Dev::tx());
    let mut d = driver(&rx, &tx);
    let (_, reply) = call(&mut d, OP_RX_PACKET, &[]);
    assert_eq!(status(&reply.unwrap()), E_AGAIN);
}
