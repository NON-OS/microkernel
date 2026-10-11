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

//! The return ring in host memory: what the part's entries hand up, and
//! the entries a hostile part writes that must hand up nothing, each
//! consumed once.

use crate::constants::desc::*;
use crate::constants::layout::*;
use crate::constants::ring::*;
use crate::queue::ret::Rx;

use super::memory::{Memory, RX_BUF_PHYS};

fn ret(m: &mut Memory, at: usize, opaque: u32, len: u32, err: u32) {
    let bd = RET_RING_AT + at * RX_BD_BYTES;
    m.put32(bd + 8, len);
    m.put32(bd + 20, err);
    m.put32(bd + 28, opaque);
}

#[test]
fn a_frame_is_handed_up_once_and_its_buffer_reposted() {
    let mut m = Memory::new();
    let mut q = m.rx_queue();
    q.init();
    ret(&mut m, 0, RXD_OPAQUE_RING_STD | 5, 64, 0);
    ret(&mut m, 1, RXD_OPAQUE_RING_STD | 5, 64, 0);
    assert_eq!(q.consume(2), Some(Rx::Frame { buf: 5, len: 60 }));
    // The slot was already taken: the replay hands up nothing.
    assert_eq!(q.consume(2), Some(Rx::Skipped));
    assert_eq!(q.consume(2), None);
    q.post(5);
    let bd = STD_RING_AT + RX_BUFFERS * RX_BD_BYTES;
    assert_eq!(m.get32(bd + 4), (RX_BUF_PHYS + 5 * RX_BUFFER_LEN as u64) as u32);
    assert_eq!(q.std_prod as usize, RX_BUFFERS + 1);
}

#[test]
fn hostile_producer_and_unposted_slots_hand_up_nothing() {
    let mut m = Memory::new();
    let mut q = m.rx_queue();
    q.init();
    for hw in [RX_RET_RING_SIZE as u16, 0xFFFF] {
        assert_eq!(q.consume(hw), None);
        assert_eq!(q.ret_cons, 0);
    }
    ret(&mut m, 0, RXD_OPAQUE_RING_STD | 300, 64, 0);
    ret(&mut m, 1, 0x2_0000 | 1, 64, 0);
    ret(&mut m, 2, RXD_OPAQUE_RING_STD | 9, 64, RXD_ERR_BAD_CRC);
    assert_eq!(q.consume(3), Some(Rx::Skipped));
    assert_eq!(q.consume(3), Some(Rx::Skipped));
    assert_eq!(q.consume(3), Some(Rx::Dropped { buf: 9 }));
    assert_eq!(q.ret_cons, 3);
}
