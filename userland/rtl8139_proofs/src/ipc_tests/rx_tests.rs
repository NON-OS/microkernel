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

//! The receive handler over the part's ring: a good frame comes back whole
//! behind its length and without its CRC, an empty ring answers "again",
//! and a header no good frame carries, or a receive error the part latched,
//! answers E_IO with nothing copied.

use super::serve::{ask, serve, status};
use crate::constants::dma::RX_BUF_BYTES;
use crate::constants::regs::{CMD_RX_BUF_EMPTY, ISR_RX_ERR, REG_CMD, REG_ISR, RX_STATUS_OK};
use crate::part::{driver_over, part};
use crate::protocol::{E_AGAIN, E_IO, OP_RX_PACKET, RESP_HDR_LEN, STATUS_LEN};

/// A ring with one header at offset 0, then `frame` bytes counting up.
fn ring_with(status: u16, raw_len: u16, frame: usize) -> Box<[u8; RX_BUF_BYTES]> {
    let mut ring = Box::new([0u8; RX_BUF_BYTES]);
    ring[0..2].copy_from_slice(&status.to_le_bytes());
    ring[2..4].copy_from_slice(&raw_len.to_le_bytes());
    (0..frame).for_each(|i| ring[4 + i] = i as u8);
    ring
}

#[test]
fn a_good_frame_comes_back_whole_behind_its_length() {
    let _part = part(false);
    let ring = ring_with(RX_STATUS_OK, 1514 + 4, 1514 + 4);
    let mut d = driver_over(ring.as_ptr() as u64);
    let got = serve(&mut d, &[ask(OP_RX_PACKET, &[])]);
    let body = &got[0].1[RESP_HDR_LEN + STATUS_LEN..];
    assert_eq!(status(&got[0].1), 0);
    assert_eq!(body[..4], 1514u32.to_le_bytes(), "the length leads");
    assert_eq!(body.len(), 4 + 1514, "and the CRC is not sent on");
    assert!(body[4..].iter().enumerate().all(|(i, b)| *b == i as u8), "the whole frame");
}

#[test]
fn an_empty_ring_answers_again_and_a_bad_one_answers_io() {
    let _part = part(false);
    let ring = ring_with(0, 68, 0);
    let mut d = driver_over(ring.as_ptr() as u64);
    d.pio.w8(REG_CMD, CMD_RX_BUF_EMPTY).expect("buffer empty");
    assert_eq!(status(&serve(&mut d, &[ask(OP_RX_PACKET, &[])])[0].1), E_AGAIN);
    d.pio.w8(REG_CMD, 0).expect("a frame waiting");
    let bad = serve(&mut d, &[ask(OP_RX_PACKET, &[])]);
    assert_eq!((status(&bad[0].1), bad[0].1.len()), (E_IO, RESP_HDR_LEN + STATUS_LEN));
    d.pio.w16(REG_ISR, ISR_RX_ERR).expect("an error latched");
    assert_eq!(status(&serve(&mut d, &[ask(OP_RX_PACKET, &[])])[0].1), E_IO);
}
