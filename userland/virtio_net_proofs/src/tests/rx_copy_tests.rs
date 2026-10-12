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

//! A frame taken off the ring is the driver's own copy, and the buffer it
//! came from goes back to the device wiped: whatever the device writes next
//! cannot reach a frame already handed out, and nothing of a delivered
//! frame stays in DMA memory.

use super::harness::Dev;
use crate::constants::{RX_BUFFER_LEN, VIRTIO_NET_HDR_LEN};
use crate::rx::take_one;

const SLOT: usize = 5;
const START: usize = SLOT * RX_BUFFER_LEN as usize;

#[test]
fn a_taken_frame_does_not_change_when_the_device_rewrites_its_slot() {
    let dev = Dev::rx();
    let mut rx = dev.rx_queue();
    dev.bufs.put(START + VIRTIO_NET_HDR_LEN, &[0xA0; 64]);
    dev.complete(0, SLOT as u32, (VIRTIO_NET_HDR_LEN + 64) as u32);
    let mut frame = [0u8; 64];
    assert_eq!(take_one(&mut rx, &mut frame), Some(64));
    // The device owns the buffer again and fills it with the next frame.
    dev.bufs.put(START + VIRTIO_NET_HDR_LEN, &[0x55; 64]);
    assert!(frame.iter().all(|&b| b == 0xA0), "the device rewrote a delivered frame");
}

#[test]
fn the_buffer_goes_back_wiped_of_header_and_frame() {
    let dev = Dev::rx();
    let mut rx = dev.rx_queue();
    dev.bufs.put(START, &[0xEE; 200]);
    dev.complete(0, SLOT as u32, 150);
    assert_eq!(take_one(&mut rx, &mut [0u8; 1514]), Some(140));
    assert!((0..150).all(|i| dev.bufs.get(START + i) == 0), "the frame stayed behind");
    assert_eq!(dev.bufs.get(START + 150), 0xEE, "only what the device wrote is wiped");
}

#[test]
fn a_dropped_frame_is_wiped_too_and_the_wipe_stays_in_its_buffer() {
    let dev = Dev::rx();
    let mut rx = dev.rx_queue();
    let next = START + RX_BUFFER_LEN as usize;
    dev.bufs.put(START, &[0xEE; 2048]);
    dev.bufs.put(next, &[0x77; 4]);
    dev.complete(0, SLOT as u32, u32::MAX);
    assert_eq!(take_one(&mut rx, &mut [0u8; 1514]), Some(0));
    assert!((0..2048).all(|i| dev.bufs.get(START + i) == 0), "an oversized frame stayed");
    assert_eq!(dev.bufs.get(next), 0x77, "the wipe ran into the next buffer");
}
