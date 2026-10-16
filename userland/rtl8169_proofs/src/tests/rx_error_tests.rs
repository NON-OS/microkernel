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

//! A whole frame the MAC completed with its error summary set (RxRES, with
//! CRC, runt or watchdog under it) is dropped as Linux rtl_rx drops it: no
//! copy, the cursor moved on, and the slot handed back to the part naming
//! its own buffer again.

use super::memory::{Memory, RX_BUF_DA};
use super::model::window;
use crate::constants::queue::BUFFER_SIZE;
use crate::constants::regs::{DESC_FS, DESC_LS, DESC_OWN};
use crate::queue::desc::{desc, desc_mut, Descriptor};
use crate::rx::recv_one;

const RX_RES: u32 = 1 << 21;
const ERRORS: [u32; 4] = [RX_RES, RX_RES | 1 << 19, RX_RES | 1 << 20, RX_RES | 1 << 22];

#[test]
fn a_frame_with_its_error_summary_set_is_dropped_and_the_ring_moves_on() {
    for err in ERRORS {
        let bar = window();
        let mut mem = Memory::new();
        let mut d = mem.driver(&bar);
        let bad =
            Descriptor { opts1: DESC_FS | DESC_LS | err | 64, opts2: 0, addr_lo: 0, addr_hi: 0 };
        // SAFETY: slot 0 is inside the ring `mem` holds.
        unsafe { desc_mut(d.rx.desc_va, 0, bad) };
        let mut out = [0xEEu8; 2048];
        assert_eq!(recv_one(&mut d, &mut out), Err("rtl8169 rx descriptor error"), "{err:#x}");
        assert!(out.iter().all(|b| *b == 0xEE), "nothing copied");
        assert_eq!(d.rx.cur, 1);
        // SAFETY: as above.
        let back = unsafe { desc(d.rx.desc_va, 0) };
        assert_eq!(back.opts1, DESC_OWN | BUFFER_SIZE as u32, "handed back");
        assert_eq!((back.addr_lo, back.addr_hi), (RX_BUF_DA as u32, (RX_BUF_DA >> 32) as u32));
    }
}
