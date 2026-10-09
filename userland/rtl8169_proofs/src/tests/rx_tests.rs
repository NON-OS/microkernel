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

//! Receive against descriptors the part wrote, good and hostile. The length
//! and the flags are the device's word; a descriptor that does not describe
//! one whole frame that fits is dropped, its slot handed back, and the cursor
//! moved past it, and nothing is copied for it.

use super::memory::Memory;
use super::model::window;
use crate::constants::queue::{BUFFER_SIZE, RX_DESC_COUNT};
use crate::constants::regs::{DESC_EOR, DESC_FS, DESC_LS, DESC_OWN};
use crate::constants::MAX_ETHERNET_FRAME;
use crate::queue::desc::{desc, desc_mut, Descriptor};
use crate::rx::recv_one;

const WHOLE: u32 = DESC_FS | DESC_LS;

fn written(opts1: u32) -> Descriptor {
    Descriptor { opts1, opts2: 0, addr_lo: 0, addr_hi: 0 }
}

/// Every slot handed back to the part: owned by it, sized to the buffer,
/// with the ring closed at the last one.
fn rearmed(d: Descriptor, idx: usize) -> bool {
    let eor = if idx == RX_DESC_COUNT - 1 { DESC_EOR } else { 0 };
    d.opts1 == DESC_OWN | eor | BUFFER_SIZE as u32
}

#[test]
fn a_slot_the_part_still_owns_is_not_read() {
    let bar = window();
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar);
    unsafe { desc_mut(d.rx.desc_va, 0, written(DESC_OWN | WHOLE | 64)) };
    let mut out = [0u8; 2048];
    assert_eq!(recv_one(&mut d, &mut out), Ok(None));
    assert_eq!(d.rx.cur, 0);
}

#[test]
fn a_whole_frame_is_copied_without_its_crc_and_the_slot_handed_back() {
    let bar = window();
    let mut mem = Memory::new();
    for (i, b) in mem.rx_buf.iter_mut().take(64).enumerate() {
        *b = i as u8;
    }
    let mut d = mem.driver(&bar);
    unsafe { desc_mut(d.rx.desc_va, 0, written(WHOLE | 64)) };
    let mut out = [0xEEu8; 2048];
    assert_eq!(recv_one(&mut d, &mut out), Ok(Some(60)));
    assert!(out[..60].iter().enumerate().all(|(i, b)| *b == i as u8));
    assert_eq!(out[60], 0xEE, "the CRC was not copied");
    assert_eq!(d.rx.cur, 1);
    assert!(rearmed(unsafe { desc(d.rx.desc_va, 0) }, 0));
}

#[test]
fn hostile_lengths_are_dropped_without_a_copy_and_the_ring_moves_on() {
    let lengths = [
        0u32,
        1,
        4,
        BUFFER_SIZE as u32 + 1,
        0x3FFF,
        MAX_ETHERNET_FRAME as u32 + 5,
    ];
    for len in lengths {
        let bar = window();
        let mut mem = Memory::new();
        let mut d = mem.driver(&bar);
        unsafe { desc_mut(d.rx.desc_va, 0, written(WHOLE | len)) };
        let mut out = [0xEEu8; 2048];
        assert!(recv_one(&mut d, &mut out).is_err(), "length {len} was trusted");
        assert!(out.iter().all(|b| *b == 0xEE), "length {len} copied something");
        assert_eq!(d.rx.cur, 1, "length {len} left the cursor behind the part");
        assert!(rearmed(unsafe { desc(d.rx.desc_va, 0) }, 0), "length {len} lost its slot");
    }
}

#[test]
fn a_fragment_is_dropped_not_reassembled() {
    for flags in [0, DESC_FS, DESC_LS] {
        let bar = window();
        let mut mem = Memory::new();
        let mut d = mem.driver(&bar);
        unsafe { desc_mut(d.rx.desc_va, 0, written(flags | 64)) };
        let mut out = [0xEEu8; 2048];
        assert!(recv_one(&mut d, &mut out).is_err());
        assert!(out.iter().all(|b| *b == 0xEE));
        assert_eq!(d.rx.cur, 1);
    }
}

#[test]
fn a_frame_longer_than_the_callers_buffer_is_dropped_not_truncated_into_it() {
    let bar = window();
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar);
    unsafe { desc_mut(d.rx.desc_va, 0, written(WHOLE | 1004)) };
    let mut out = [0xEEu8; 64];
    assert!(recv_one(&mut d, &mut out).is_err());
    assert!(out.iter().all(|b| *b == 0xEE));
}

#[test]
fn the_last_slot_is_handed_back_with_the_ring_closed() {
    let bar = window();
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar);
    let last = RX_DESC_COUNT - 1;
    d.rx.cur = last;
    unsafe { desc_mut(d.rx.desc_va, last, written(WHOLE | 64)) };
    let mut out = [0u8; 2048];
    assert_eq!(recv_one(&mut d, &mut out), Ok(Some(60)));
    assert_eq!(d.rx.cur, 0, "the cursor wraps");
    assert!(rearmed(unsafe { desc(d.rx.desc_va, last) }, last));
}
