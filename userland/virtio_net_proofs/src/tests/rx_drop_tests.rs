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

//! OP_RX_BATCH and the entries take_one drops: none of them reaches a batch
//! as a frame, every one is still taken off the ring, and a batch is never
//! longer than its limits whatever lengths the device reports.

use super::drive::driver;
use super::fuzz::{hostile_len, rounds};
use super::harness::Dev;
use super::rx_batch_tests::frames;
use crate::constants::{MAX_ETHERNET_FRAME, VIRTIO_NET_HDR_LEN};
use crate::protocol::{BATCH_MAX_BYTES, BATCH_MAX_FRAMES};
use crate::server::handlers::rx_fill::fill;

const HDR: u32 = VIRTIO_NET_HDR_LEN as u32;

#[test]
fn a_dropped_entry_is_no_frame_in_the_batch() {
    let (rx, tx) = (Dev::rx(), Dev::tx());
    let mut d = driver(&rx, &tx);
    // A frame, an id naming no buffer, a header with no frame, a frame.
    let entries = [(0, HDR + 60), (9_999, HDR + 60), (2, HDR), (3, HDR + 98)];
    for (n, &(id, len)) in entries.iter().enumerate() {
        rx.complete(n as u16, id, len);
    }
    let body = fill(&mut d.rx).expect("two frames were waiting");
    assert_eq!(frames(&body), [60, 98]);
    assert_eq!(d.rx.last_used, 4, "the dropped entries were taken too");
}

#[test]
fn a_ring_of_dropped_entries_alone_gives_no_batch() {
    let (rx, tx) = (Dev::rx(), Dev::tx());
    let mut d = driver(&rx, &tx);
    for n in 0..3u16 {
        rx.complete(n, u32::from(n), u32::MAX);
    }
    assert!(fill(&mut d.rx).is_none(), "nothing to hand on");
    assert_eq!(d.rx.last_used, 3);
}

#[test]
fn hostile_lengths_never_make_a_batch_too_long() {
    let (rx, tx) = (Dev::rx(), Dev::tx());
    let mut d = driver(&rx, &tx);
    let mut n = 0u16;
    for seed in 1..rounds(2_000) {
        let mut s = seed;
        for _ in 0..(seed % 64) {
            rx.complete(n, u32::from(n % 64), hostile_len(&mut s));
            n = n.wrapping_add(1);
        }
        while let Some(body) = (d.rx.last_used != n).then(|| fill(&mut d.rx)) {
            let Some(body) = body else { continue };
            assert!(body.len() <= BATCH_MAX_BYTES);
            let lens = frames(&body);
            assert!(lens.len() <= BATCH_MAX_FRAMES);
            assert!(lens.iter().all(|&l| l > 0 && l <= MAX_ETHERNET_FRAME), "{lens:?}");
        }
    }
}
