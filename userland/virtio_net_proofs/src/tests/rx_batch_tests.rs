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

//! OP_RX_BATCH: whatever lengths the device reports, a batch never passes
//! BATCH_MAX_BYTES (net.core's receive buffer), so a frame too long can no
//! longer cost the good frames taken off the ring with it.

use super::drive::driver;
use super::fuzz::{hostile_len, rounds};
use super::harness::Dev;
use crate::constants::{MAX_ETHERNET_FRAME, VIRTIO_NET_HDR_LEN};
use crate::protocol::{BATCH_MAX_BYTES, BATCH_MAX_FRAMES};
use crate::server::handlers::rx_fill::fill;

const HDR: u32 = VIRTIO_NET_HDR_LEN as u32;

/// The frame lengths a batch body carries, checked against its own count.
fn frames(body: &[u8]) -> Vec<usize> {
    let count = u32::from_le_bytes(body[..4].try_into().unwrap()) as usize;
    let (mut at, mut out) = (4, Vec::new());
    while at < body.len() {
        let len = u32::from_le_bytes(body[at..at + 4].try_into().unwrap()) as usize;
        out.push(len);
        at += 4 + len;
    }
    assert_eq!((at, out.len()), (body.len(), count), "the body does not parse");
    out
}

#[test]
fn oversized_entries_at_the_edge_of_the_room_check_stay_inside_the_limit() {
    let (rx, tx) = (Dev::rx(), Dev::tx());
    let mut d = driver(&rx, &tx);
    for i in 0..33u16 {
        let len = if i == 31 { HDR + 708 } else { u32::MAX };
        rx.complete(i, u32::from(i % 64), len);
    }
    let body = fill(&mut d.rx).expect("frames were waiting");
    assert!(body.len() <= BATCH_MAX_BYTES, "a {}-byte batch", body.len());
    assert_eq!(frames(&body).iter().filter(|&&l| l > 0).count(), 1, "the one real frame");
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
        while let Some(body) = fill(&mut d.rx) {
            assert!(body.len() <= BATCH_MAX_BYTES);
            let lens = frames(&body);
            assert!(lens.len() <= BATCH_MAX_FRAMES);
            assert!(lens.iter().all(|&l| l <= MAX_ETHERNET_FRAME));
        }
        assert_eq!(d.rx.last_used, n, "every entry was taken");
    }
}
