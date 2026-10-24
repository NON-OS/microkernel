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

//! A frame longer than one buffer, as a hostile part could write it: every
//! piece without EOP and the piece with EOP that ends it are let go, none
//! joined or handed up alone, and each slot let go is counted; the next
//! whole frame is taken.

use crate::facts::rx::{RXD_STAT_DD, RXD_STAT_EOP};
use crate::queue::layout::RxDesc;

use super::memory::Memory;

fn wb(staterr: u32, length: u16) -> RxDesc {
    RxDesc { buffer_addr: !0, staterr, length, vlan: 0 }
}

#[test]
fn every_piece_of_a_long_frame_is_let_go_through_its_end() {
    let mut mem = Memory::new();
    let (piece, end) = (RXD_STAT_DD, RXD_STAT_DD | RXD_STAT_EOP);
    mem.rx[0] = wb(piece, 2048);
    mem.rx[1] = wb(piece, 2048);
    mem.rx[2] = wb(end, 100);
    mem.rx[3] = wb(end, 60);
    mem.rx[4] = wb(end | 1 << 24, 60);
    let mut rx = mem.rx_ring();
    let got: Vec<_> = (0..5).map(|_| rx.consume()).collect();
    assert_eq!(got, [Some((0, 0)), Some((1, 0)), Some((2, 0)), Some((3, 60)), Some((4, 0))]);
    assert_eq!((rx.dropped, rx.piecing), (4, false));
}

#[test]
fn a_piece_whose_end_never_comes_keeps_being_let_go() {
    let mut mem = Memory::new();
    for d in mem.rx.iter_mut() {
        *d = wb(RXD_STAT_DD, 2048);
    }
    let mut rx = mem.rx_ring();
    let n = mem.rx.len();
    for _ in 0..n {
        assert_eq!(rx.consume().map(|(_, len)| len), Some(0));
    }
    assert_eq!((rx.dropped as usize, rx.piecing), (n, true));
}
