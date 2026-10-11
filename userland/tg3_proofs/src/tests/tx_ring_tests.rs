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

//! The send ring in host memory: the BD a frame gets, the slot its bytes
//! go in, and at most TX_BUFFERS - 1 frames in flight.

use crate::constants::desc::*;
use crate::constants::layout::*;

use super::memory::{Memory, TX_BUF_PHYS};

#[test]
fn a_frame_gets_one_bd_with_its_length_and_end() {
    let mut m = Memory::new();
    let mut t = m.tx_ring();
    t.init();
    let first = t.next_buffer_va();
    assert_eq!(t.post(60), 1);
    assert_eq!(t.next_buffer_va(), first + TX_BUFFER_LEN as u64);
    assert_eq!(m.get32(TX_RING_AT), (TX_BUF_PHYS >> 32) as u32);
    assert_eq!(m.get32(TX_RING_AT + 4), TX_BUF_PHYS as u32);
    assert_eq!(m.get32(TX_RING_AT + 8), 60 << TXD_LEN_SHIFT | TXD_FLAG_END);
    assert_eq!(m.get32(TX_RING_AT + 12), 0);
}

#[test]
fn the_ring_fills_one_short_of_its_slots() {
    let mut m = Memory::new();
    let mut t = m.tx_ring();
    t.init();
    for n in 0..TX_BUFFERS - 1 {
        assert!(!t.full(), "{n}");
        t.post(60);
    }
    assert!(t.full());
    t.reclaim(1);
    assert!(!t.full());
    assert_eq!(t.in_flight(), TX_BUFFERS - 2);
}
