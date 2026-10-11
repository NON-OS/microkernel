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

//! One return-ring entry against tg3_rx: the length less the FCS must hold
//! a frame, the opaque word must name the standard ring and a slot in it,
//! and any RXD_ERR_MASK bit drops it.

use crate::constants::desc::*;
use crate::constants::ring::RX_STD_RING_SIZE;
use crate::constants::{FCS_LEN, MAX_ETHERNET_FRAME, MIN_ETHERNET_FRAME};
use crate::queue::rx_parse::{parse, Entry};

const STD: u32 = RXD_OPAQUE_RING_STD;

#[test]
fn bounds_of_a_good_frame() {
    let min = (MIN_ETHERNET_FRAME + FCS_LEN) as u32;
    let max = (MAX_ETHERNET_FRAME + FCS_LEN) as u32;
    assert_eq!(parse(STD | 7, min, 0), Entry::Frame(7, MIN_ETHERNET_FRAME as u16));
    assert_eq!(parse(STD | 7, max, 0), Entry::Frame(7, MAX_ETHERNET_FRAME as u16));
    for len in [0, 1, 3, 4, min - 1, max + 1, 0xFFFF] {
        assert_eq!(parse(STD | 7, len, 0), Entry::Drop(7), "{len}");
    }
    // The index half of idx_len is not the length.
    assert_eq!(parse(STD | 7, 0xABCD_0000 | min, 0), Entry::Frame(7, 14));
}

#[test]
fn other_rings_and_far_indexes_are_foreign() {
    for ring in [0, 0x2_0000, 0x3_0000, 0x4_0000, 0x7_0000] {
        assert_eq!(parse(ring | 1, 64, 0), Entry::Foreign, "{ring:#x}");
    }
    for idx in [RX_STD_RING_SIZE as u32, 0x200, 0xFFFF] {
        assert_eq!(parse(STD | idx, 64, 0), Entry::Foreign, "{idx}");
    }
    assert_eq!(parse(STD | 511, 64, 0), Entry::Frame(511, 60));
}

#[test]
fn every_error_bit_drops_and_the_odd_nibble_does_not() {
    for bit in 16..25 {
        let e = 1u32 << bit;
        let want = if e & RXD_ERR_MASK != 0 { Entry::Drop(3) } else { Entry::Frame(3, 60) };
        assert_eq!(parse(STD | 3, 64, e), want, "bit {bit}");
    }
    assert_eq!(parse(STD | 3, 64, 0x0010_0000), Entry::Frame(3, 60));
}
