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

//! The receive handler against what the part writes back: a descriptor that
//! is split, errored, empty or longer than a frame answers E_IO and its slot
//! is handed back in read format; a good frame is copied out whole behind
//! its length; and RDT only ever names a slot already copied out.

use super::memory::{Memory, RX_BUF_PHYS};
use super::model::window;
use super::serve::{ask, pools, serve, status};
use crate::constants::queue::RX_BUFFER_LEN;
use crate::constants::regs::REG_RDT;
use crate::constants::rx_bits::{RXDEXT_STATERR_RXE as RXE, RXD_STAT_DD as DD, RXD_STAT_EOP};
use crate::protocol::{E_AGAIN, E_IO, OP_RX_PACKET, RESP_HDR_LEN, STATUS_LEN};
use crate::queue::layout::RxDesc;

const DONE: u32 = DD | RXD_STAT_EOP;

#[test]
fn hostile_write_backs_answer_io_and_their_slots_go_back() {
    let bar = window();
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar);
    let _pools = pools(&mut d);
    let cases = [(DD, 60), (DONE | RXE, 60), (DONE, 0), (DONE, 1515), (DONE, 2048), (DONE, 0xFFFF)];
    for (i, (staterr, length)) in cases.into_iter().enumerate() {
        mem.rx[i] = RxDesc { buffer_addr: 0xDEAD, staterr, length, vlan: 0 };
        let got = serve(&mut d, &[ask(OP_RX_PACKET, &[])]);
        assert_eq!(status(&got[0].1), E_IO, "case {i}");
        assert_eq!(bar.wrote32(REG_RDT), i as u32, "slot {i} handed back");
        let back = mem.rx[i];
        assert_eq!((back.buffer_addr, back.staterr), (RX_BUF_PHYS + i as u64 * 2048, 0));
    }
}

#[test]
fn a_good_frame_is_copied_out_whole_before_its_slot_goes_back() {
    let bar = window();
    bar.present32(REG_RDT, 31);
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar);
    let (mut rx_pool, _tx_pool) = pools(&mut d);
    let idle = serve(&mut d, &[ask(OP_RX_PACKET, &[])]);
    assert_eq!((status(&idle[0].1), bar.wrote32(REG_RDT)), (E_AGAIN, 31), "nothing yet");
    for (slot, len) in [(0, 1514), (1, 14)] {
        let fill = 0x5A + slot as u8;
        rx_pool[slot * RX_BUFFER_LEN..][..len].fill(fill);
        mem.rx[slot] = RxDesc { buffer_addr: 0, staterr: DONE, length: len as u16, vlan: 0 };
        let got = serve(&mut d, &[ask(OP_RX_PACKET, &[])]);
        let body = &got[0].1[RESP_HDR_LEN + STATUS_LEN..];
        assert_eq!(status(&got[0].1), 0);
        assert_eq!(body[..4], (len as u32).to_le_bytes(), "the length leads");
        assert_eq!(body[4..], vec![fill; len][..], "then the whole frame, nothing more");
        assert_eq!(bar.wrote32(REG_RDT), slot as u32, "the slot just copied, not past it");
    }
}
