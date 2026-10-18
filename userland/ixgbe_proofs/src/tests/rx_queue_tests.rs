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
//! Receive queue 0 at 0x1000: the queue seen disabled first, SRRCTL (at
//! 0x2100) written whole for one advanced buffer of 2 KiB, every descriptor
//! in read format, and slots handed to the part only once the enable reads
//! back.
//!
//! Not proven here: a queue whose enable never reads back. The model shares
//! memory with the driver, so the driver's own write reads back before any
//! concurrent model can undo it; that refusal rests on `until`, whose
//! timeout is proven in wait_tests.

use crate::bring::rings::rx::program;
use crate::facts::regs::*;
use crate::queue::layout::RxDesc;
use crate::sizes::ring::{RX_BUFFER_LEN, RX_DESC_COUNT};

use super::memory::{Memory, RX_BUF_PHYS};
use super::model::{regs, window};

const RING: u64 = 0x0000_0002_0000_8000;

#[test]
fn queue_zero_is_programmed_for_one_advanced_buffer_then_filled() {
    let bar = window();
    let mut mem = Memory::new();
    mem.rx[5] = RxDesc { buffer_addr: 0, staterr: 3, length: 9, vlan: 9 };
    bar.present32(REG_SRRCTL, 0x8000_0000 | 0x0E00_3F7F);
    bar.present32(REG_RXDCTL, (1 << 25) | 0x0000_0123);
    assert_eq!(program(&regs(&bar), &mem.rx_ring(), RING), Ok(()));
    assert_eq!(bar.wrote32(0x1000), 0x0000_8000, "RDBAL");
    assert_eq!(bar.wrote32(0x1004), 0x0000_0002, "RDBAH");
    assert_eq!(bar.wrote32(0x1008), 512, "RDLEN");
    assert_eq!(bar.wrote32(REG_RDH), 0);
    let srrctl = bar.wrote32(0x2100);
    assert_eq!(srrctl & 0x1F, 2, "BSIZEPKT 2 KiB");
    assert_eq!((srrctl >> 8) & 0x3F, 4, "BSIZEHDRSIZE 256 bytes");
    assert_eq!((srrctl >> 25) & 0x7, 1, "DESCTYPE advanced one buffer");
    assert_eq!(srrctl, 0x0200_0402, "whole: the DROP_EN and the rest left are gone");
    assert_eq!(bar.wrote32(0x1028), (1 << 25) | 0x123, "enabled, thresholds as found");
    assert_eq!(bar.wrote32(0x1018), RX_DESC_COUNT as u32 - 1, "RDT: all but one slot");
    for (i, d) in mem.rx.iter().enumerate() {
        assert_eq!(d.buffer_addr, RX_BUF_PHYS + (i * RX_BUFFER_LEN) as u64);
        assert_eq!((d.staterr, d.length, d.vlan), (0, 0, 0), "hdr_addr zero");
    }
}
