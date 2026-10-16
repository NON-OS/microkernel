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

//! The receive handler over what the part writes: a slot still the part's
//! answers "again"; a descriptor that is split, carries the error summary,
//! or holds a length no frame has answers E_IO with nothing copied; and a
//! good frame comes back whole behind its length, without its CRC.

use super::memory::Memory;
use super::model::window;
use super::serve::{ask, serve, status};
use crate::constants::regs::{DESC_FS, DESC_LS, DESC_OWN, DESC_RX_RES};
use crate::protocol::{E_AGAIN, E_IO, OP_RX_PACKET, RESP_HDR_LEN, STATUS_LEN};
use crate::queue::desc::{desc_mut, Descriptor};

fn written(opts1: u32) -> Descriptor {
    Descriptor { opts1, opts2: 0, addr_lo: 0, addr_hi: 0 }
}

#[test]
fn hostile_descriptors_answer_io_and_a_held_one_answers_again() {
    let bar = window();
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar);
    let whole = DESC_FS | DESC_LS;
    let cases = [DESC_FS | 64, DESC_LS | 64, whole | DESC_RX_RES | 64, whole | 4, whole | 0x3FFF];
    for (i, opts1) in cases.into_iter().enumerate() {
        // SAFETY: slot `i` is inside the ring `mem` holds.
        unsafe { desc_mut(d.rx.desc_va, i, written(opts1)) };
        let got = serve(&mut d, &[ask(OP_RX_PACKET, &[])]);
        assert_eq!((status(&got[0].1), got[0].1.len()), (E_IO, RESP_HDR_LEN + STATUS_LEN));
    }
    // SAFETY: as above.
    unsafe { desc_mut(d.rx.desc_va, cases.len(), written(DESC_OWN | 64)) };
    assert_eq!(status(&serve(&mut d, &[ask(OP_RX_PACKET, &[])])[0].1), E_AGAIN);
}

#[test]
fn a_good_frame_comes_back_whole_behind_its_length() {
    let bar = window();
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar);
    (0..1518).for_each(|i| mem.rx_buf[i] = i as u8);
    // SAFETY: slot 0 is inside the ring `mem` holds.
    unsafe { desc_mut(d.rx.desc_va, 0, written(DESC_FS | DESC_LS | 1518)) };
    let got = serve(&mut d, &[ask(OP_RX_PACKET, &[])]);
    let body = &got[0].1[RESP_HDR_LEN + STATUS_LEN..];
    assert_eq!(status(&got[0].1), 0);
    assert_eq!(body[..4], 1514u32.to_le_bytes(), "the length leads, without the CRC");
    assert_eq!(body.len(), 4 + 1514, "and the CRC is not sent on");
    assert!(body[4..].iter().enumerate().all(|(i, b)| *b == i as u8), "the whole frame");
}
