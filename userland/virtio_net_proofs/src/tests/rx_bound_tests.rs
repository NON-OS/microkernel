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

//! The receive bounds, every used length from nothing to past the buffer.
//!
//! This replaces a test that asserted a 2038-byte "frame" was handed on: a
//! frame that long does not fit the 1552-byte reply it is copied into, and
//! the copy panicked the capsule, which nothing restarts. What is right is
//! that a frame longer than Ethernet allows is dropped and its buffer goes
//! straight back to the device.

use super::drive::take;
use super::harness::Dev;
use crate::constants::VIRTIO_NET_HDR_LEN_V1;
use crate::constants::{MAX_ETHERNET_FRAME, RX_BUFFER_LEN, VIRTIO_NET_HDR_LEN};

#[test]
fn only_a_frame_of_one_to_1514_bytes_is_handed_on() {
    for hdr in [VIRTIO_NET_HDR_LEN, VIRTIO_NET_HDR_LEN_V1] {
        let dev = Dev::rx();
        let mut out = [0u8; MAX_ETHERNET_FRAME];
        let lens = (0..=4096u32).chain([u32::MAX - 1, u32::MAX]);
        for used_len in lens {
            let (got, posted) = take(&dev, hdr, 3, used_len, &mut out);
            let frame = (used_len as usize).saturating_sub(hdr);
            let want = if frame <= MAX_ETHERNET_FRAME { frame } else { 0 };
            assert_eq!(got, want, "hdr {hdr} used_len {used_len}");
            assert_eq!(posted, 3, "the buffer used is the one recycled");
        }
    }
}

#[test]
fn an_oversized_frame_is_dropped_and_its_buffer_recycled() {
    let dev = Dev::rx();
    let base = 3 * RX_BUFFER_LEN as usize + VIRTIO_NET_HDR_LEN;
    let pattern: Vec<u8> = (0..1600).map(|i| (i % 251) as u8 | 1).collect();
    let hdr = VIRTIO_NET_HDR_LEN as u32;
    let mut out = [0u8; MAX_ETHERNET_FRAME];
    for used_len in [hdr + 1515, hdr + 2038, u32::MAX] {
        dev.bufs.put(base, &pattern);
        assert_eq!(take(&dev, VIRTIO_NET_HDR_LEN, 3, used_len, &mut out), (0, 3));
    }
    dev.bufs.put(base, &pattern);
    assert_eq!(take(&dev, VIRTIO_NET_HDR_LEN, 3, hdr + 1514, &mut out), (1514, 3));
    assert_eq!(&out[..], &pattern[..1514], "the frame is the buffer's bytes");
}
