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

//! The receive handler against what the part writes: a descriptor that is
//! split, errored, empty or longer than a frame answers E_IO and its slot is
//! handed back; a good frame is copied out whole behind its length; and RDT
//! only ever names a slot already copied out, never one past it.

use super::serve::{ask, serve, status};
use crate::constants::queue::{RX_BUFFER_LEN, RX_STATUS_DD, RX_STATUS_EOP};
use crate::constants::regs::REG_RDT;
use crate::constants::Family;
use crate::model::memory::Memory;
use crate::model::window::i82574;
use crate::protocol::{E_AGAIN, E_IO, OP_RX_PACKET, RESP_HDR_LEN, STATUS_LEN};
use crate::queue::layout::RxDesc;

const DONE: u8 = RX_STATUS_DD | RX_STATUS_EOP;

fn written(status: u8, errors: u8, length: u16) -> RxDesc {
    RxDesc { status, errors, length, ..RxDesc::default() }
}

#[test]
fn hostile_descriptors_answer_io_and_their_slots_go_back() {
    let bar = i82574();
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar, Family::I82574, 0x10D3);
    let cases = [(RX_STATUS_DD, 0, 60), (DONE, 1, 60), (DONE, 0x80, 60), (DONE, 0, 0)];
    let long = [(DONE, 0, 1515), (DONE, 0, 2048), (DONE, 0, 0xFFFF)];
    for (i, (s, e, len)) in cases.into_iter().chain(long).enumerate() {
        mem.rx[i] = written(s, e, len);
        let got = serve(&mut d, &[ask(OP_RX_PACKET, &[])]);
        assert_eq!(status(&got[0].1), E_IO, "case {i}");
        assert_eq!(bar.wrote32(REG_RDT), i as u32, "slot {i} handed back");
        assert_eq!(mem.rx[i].status, 0, "and cleared for the part");
    }
}

#[test]
fn a_good_frame_is_copied_out_whole_before_its_slot_goes_back() {
    let bar = i82574();
    bar.present32(REG_RDT, 31);
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar, Family::I82574, 0x10D3);
    let idle = serve(&mut d, &[ask(OP_RX_PACKET, &[])]);
    assert_eq!((status(&idle[0].1), bar.wrote32(REG_RDT)), (E_AGAIN, 31), "nothing yet");
    for (slot, len) in [(0, 1514), (1, 14)] {
        let fill = 0x5A + slot as u8;
        mem.rx_buf[slot * RX_BUFFER_LEN..][..len].fill(fill);
        mem.rx[slot] = written(DONE, 0, len as u16);
        let got = serve(&mut d, &[ask(OP_RX_PACKET, &[])]);
        let body = &got[0].1[RESP_HDR_LEN + STATUS_LEN..];
        assert_eq!(status(&got[0].1), 0);
        assert_eq!(body[..4], (len as u32).to_le_bytes(), "the length leads");
        assert_eq!(body[4..], vec![fill; len][..], "then the whole frame, nothing more");
        assert_eq!(bar.wrote32(REG_RDT), slot as u32, "the slot just copied, not past it");
    }
}
