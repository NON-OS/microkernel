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

//! A batch taken off the ring is kept until its caller asks for the next
//! number, so a lost reply can be asked for again; and full-size frames
//! fill a batch to the room check and leave the rest for the next one.

use super::drive::driver;
use super::harness::Dev;
use crate::constants::{MAX_ETHERNET_FRAME, VIRTIO_NET_HDR_LEN};
use crate::protocol::{Request, BATCH_MAX_BYTES, OP_RX_BATCH};
use crate::server::error::take_sent;
use crate::server::handlers::rx_batch;
use crate::server::handlers::rx_fill::fill;

const HDR: u32 = VIRTIO_NET_HDR_LEN as u32;

#[test]
fn a_lost_batch_is_answered_again_and_only_to_its_caller() {
    let (rx, tx) = (Dev::rx(), Dev::tx());
    let mut d = driver(&rx, &tx);
    rx.complete(0, 0, HDR + 100);
    let req = Request { op: OP_RX_BATCH, flags: 0, request_id: 1, payload_len: 4 };
    let (mut kept, mut buf) = (None, vec![0u8; 64]);
    let _ = take_sent();
    for pid in [1, 1, 2] {
        rx_batch::handle(pid, &mut d, &req, &7u32.to_le_bytes(), &mut kept, &mut buf);
    }
    let sent = take_sent();
    assert_eq!(sent[0], sent[1], "the same number asks for the same batch");
    assert_ne!(sent[0], sent[2], "another caller does not get it");
}

#[test]
fn full_size_frames_fill_a_batch_to_the_room_check() {
    let (rx, tx) = (Dev::rx(), Dev::tx());
    let mut d = driver(&rx, &tx);
    for i in 0..64u16 {
        rx.complete(i, u32::from(i), HDR + MAX_ETHERNET_FRAME as u32);
    }
    let first = fill(&mut d.rx).expect("frames were waiting");
    assert_eq!(first.len(), 4 + 43 * (4 + MAX_ETHERNET_FRAME));
    assert!(first.len() <= BATCH_MAX_BYTES);
    let rest = fill(&mut d.rx).expect("the rest");
    assert_eq!(rest.len(), 4 + 21 * (4 + MAX_ETHERNET_FRAME));
    assert!(fill(&mut d.rx).is_none());
}
