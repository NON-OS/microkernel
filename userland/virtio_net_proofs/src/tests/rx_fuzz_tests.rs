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

//! Whatever id, length and ring position a hostile device reports, take_one
//! hands on at most an Ethernet frame, copied from the buffer the id names
//! after this transport's header, posts that buffer back wiped, and never
//! posts back an id that names no buffer.

use super::fuzz::{hostile_id, hostile_len, rounds, xorshift};
use super::harness::Dev;
use crate::constants::{MAX_ETHERNET_FRAME, RX_BUFFER_LEN, RX_DESC_COUNT};
use crate::constants::{VIRTIO_NET_HDR_LEN, VIRTIO_NET_HDR_LEN_V1};
use crate::rx::take_one;

/// The frame an entry should yield: after the header, inside the buffer,
/// no longer than Ethernet allows.
fn expected(used_len: u32, hdr: usize) -> usize {
    let len = (used_len as usize).saturating_sub(hdr);
    let fits = hdr + len <= RX_BUFFER_LEN as usize && len <= MAX_ETHERNET_FRAME;
    len * usize::from(fits)
}

fn hostile_entries(hdr: usize) {
    let dev = Dev::rx();
    let mut rx = dev.rx_queue().with_hdr_len(hdr);
    let mut out = [0u8; MAX_ETHERNET_FRAME];
    for seed in 1..rounds(200_000) {
        let mut s = seed;
        let (id, used_len) = (hostile_id(&mut s), hostile_len(&mut s));
        rx.last_used = xorshift(&mut s) as u16;
        let (posted, want) = (rx.next_avail, expected(used_len, hdr));
        let (start, mark) = ((id as usize % 64) * RX_BUFFER_LEN as usize + hdr, seed as u8 | 1);
        if want > 0 {
            dev.bufs.put(start, &[mark]);
            dev.bufs.put(start + want - 1, &[mark]);
        }
        dev.complete(rx.last_used, id, used_len);
        let got = take_one(&mut rx, &mut out).expect("one pending entry");
        if id >= u32::from(RX_DESC_COUNT) {
            assert_eq!(got, 0, "an id past the buffers yields no frame");
            assert_eq!(rx.next_avail, posted, "an id past the buffers is never posted back");
            continue;
        }
        assert_eq!(got, want, "id {id} used_len {used_len}");
        if want > 0 {
            assert_eq!((out[0], out[want - 1]), (mark, mark), "copied from another place");
            assert_eq!(dev.bufs.get(start), 0, "the frame stayed in the buffer");
        }
        assert_eq!(rx.next_avail, posted.wrapping_add(1));
        assert_eq!(dev.avail(posted), id as u16, "the buffer posted back is the one used");
    }
}

#[test]
fn hostile_used_entries_never_yield_a_frame_from_elsewhere_or_too_long() {
    hostile_entries(VIRTIO_NET_HDR_LEN);
}

#[test]
fn hostile_used_entries_on_a_version_1_queue_behave_the_same() {
    hostile_entries(VIRTIO_NET_HDR_LEN_V1);
}
