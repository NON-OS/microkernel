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

//! The receive gate against packet headers the part wrote, good and
//! hostile. The status word and raw length are the device's; a header no
//! good frame carries restarts receive, a frame the caller cannot take is
//! skipped, and nothing is ever copied for either.

use crate::constants::dma::RX_BUF_BYTES;
use crate::constants::regs::RX_STATUS_OK;
use crate::part::{driver_over, part};
use crate::rx::recv_one;

/// A ring in host memory with one header at offset 0: status, raw length
/// (frame plus CRC), then `frame_len` bytes counting up.
fn ring_with(status: u16, raw_len: u16, frame_len: usize) -> Box<[u8; RX_BUF_BYTES]> {
    let mut ring = Box::new([0u8; RX_BUF_BYTES]);
    ring[0..2].copy_from_slice(&status.to_le_bytes());
    ring[2..4].copy_from_slice(&raw_len.to_le_bytes());
    for i in 0..frame_len {
        ring[4 + i] = i as u8;
    }
    ring
}

#[test]
fn a_good_frame_is_copied_without_its_crc_and_the_read_position_moves_on() {
    let _part = part(false);
    let ring = ring_with(RX_STATUS_OK, 64 + 4, 64);
    let mut d = driver_over(ring.as_ptr() as u64);
    let mut out = [0xEEu8; 2048];
    assert_eq!(recv_one(&mut d, &mut out), Ok(Some(64)));
    assert!(out[..64].iter().enumerate().all(|(i, b)| *b == i as u8));
    assert_eq!(out[64], 0xEE, "the CRC was not copied");
    assert_eq!(d.rx_offset, (64 + 4 + 4 + 3) & !3, "the next header is dword aligned");
}

#[test]
fn a_frame_still_arriving_is_left_for_the_next_poll() {
    let _part = part(false);
    let ring = ring_with(RX_STATUS_OK, 0xFFF0, 0);
    let mut d = driver_over(ring.as_ptr() as u64);
    let mut out = [0xEEu8; 2048];
    assert_eq!(recv_one(&mut d, &mut out), Ok(None));
    assert_eq!(d.rx_offset, 0);
}

#[test]
fn a_header_no_good_frame_carries_restarts_receive_and_copies_nothing() {
    let hostile: [(u16, u16); 5] = [
        (0, 68),               // status without receive-OK
        (RX_STATUS_OK, 0),     // no length at all
        (RX_STATUS_OK, 7),     // shorter than any header
        (RX_STATUS_OK, 1797),  // past the longest frame the part hands up
        (RX_STATUS_OK, 0xFFFF),
    ];
    for (status, raw_len) in hostile {
        let _part = part(false);
        let ring = ring_with(status, raw_len, 0);
        let mut d = driver_over(ring.as_ptr() as u64);
        d.rx_offset = 0;
        let mut out = [0xEEu8; 2048];
        assert_eq!(
            recv_one(&mut d, &mut out),
            Err("rtl8139 rx ring restarted"),
            "status {status:#x} length {raw_len} was trusted"
        );
        assert!(out.iter().all(|b| *b == 0xEE), "length {raw_len} copied something");
        assert_eq!(d.rx_offset, 0, "receive starts over from the top of the ring");
    }
}

#[test]
fn a_frame_longer_than_the_callers_buffer_is_skipped_not_truncated_into_it() {
    let _part = part(false);
    let ring = ring_with(RX_STATUS_OK, 1000 + 4, 1000);
    let mut d = driver_over(ring.as_ptr() as u64);
    let mut out = [0xEEu8; 64];
    assert_eq!(recv_one(&mut d, &mut out), Err("rtl8139 rx frame too large"));
    assert!(out.iter().all(|b| *b == 0xEE));
    assert_eq!(d.rx_offset, (1000 + 4 + 4 + 3) & !3, "the frame was stepped over");
}
