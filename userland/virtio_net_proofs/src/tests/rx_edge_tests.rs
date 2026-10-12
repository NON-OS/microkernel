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

//! Receive edges: the space a caller offers, a header that does not fit,
//! an idle ring, an id that names no buffer.

use super::drive::take;
use super::harness::Dev;
use crate::constants::{MAX_ETHERNET_FRAME, RX_BUFFER_LEN, RX_DESC_COUNT, VIRTIO_NET_HDR_LEN};
use crate::rx::take_one;

#[test]
fn a_frame_longer_than_the_space_offered_is_dropped() {
    let dev = Dev::rx();
    let mut small = [0u8; 64];
    assert_eq!(take(&dev, VIRTIO_NET_HDR_LEN, 5, 10 + 65, &mut small), (0, 5));
    assert_eq!(take(&dev, VIRTIO_NET_HDR_LEN, 5, 10 + 64, &mut small), (64, 5));
}

#[test]
fn more_room_than_a_frame_never_lets_a_longer_one_through() {
    let dev = Dev::rx();
    let mut room = [0u8; 4096];
    for used_len in [10 + 1515, 10 + 2038] {
        assert_eq!(take(&dev, VIRTIO_NET_HDR_LEN, 2, used_len, &mut room), (0, 2));
    }
    let max = (10 + MAX_ETHERNET_FRAME) as u32;
    assert_eq!(take(&dev, VIRTIO_NET_HDR_LEN, 2, max, &mut room), (MAX_ETHERNET_FRAME, 2));
}

#[test]
fn a_header_length_past_the_buffer_yields_nothing() {
    let dev = Dev::rx();
    let mut out = [0u8; MAX_ETHERNET_FRAME];
    for used_len in [0u32, 64, RX_BUFFER_LEN, RX_BUFFER_LEN + 1, u32::MAX] {
        assert_eq!(take(&dev, usize::MAX, 9, used_len, &mut out), (0, 9));
    }
}

#[test]
fn an_idle_ring_yields_nothing() {
    let dev = Dev::rx();
    let mut rx = dev.rx_queue();
    rx.last_used = 7;
    dev.complete(6, 0, 100);
    assert_eq!(take_one(&mut rx, &mut [0u8; 64]), None);
}

#[test]
fn an_id_that_names_no_buffer_is_dropped_and_never_posted_back() {
    let n = u32::from(RX_DESC_COUNT);
    for wild in [n, n + 3, 0xFFFF, 0x1_0000 + 3, u32::MAX] {
        let dev = Dev::rx();
        let mut rx = dev.rx_queue();
        dev.complete(0, wild, 64);
        assert_eq!(take_one(&mut rx, &mut [0u8; 64]), Some(0), "id {wild:#x}");
        assert_eq!((rx.last_used, rx.next_avail), (1, 0), "id {wild:#x} was posted back");
    }
    assert_eq!(take(&Dev::rx(), VIRTIO_NET_HDR_LEN, n - 1, 100, &mut [0u8; 90]), (90, 63));
}
