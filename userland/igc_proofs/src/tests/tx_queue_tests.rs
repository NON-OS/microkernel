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

//! igc_configure_tx_ring for queue 0 at 0xE000.
//!
//! An enable that never reads back, or reads back late, is in
//! queue_enable_tests, where the part acts on the driver's looks at the
//! clock, between its write and its first read.

use crate::constants::regs::*;
use crate::init::tx_queue::{enable, program};
use crate::queue::layout::TxDesc;

use super::memory::Memory;
use super::model::{regs, window};

const RING: u64 = 0x0000_0001_2345_6000;

#[test]
fn queue_zero_is_programmed_at_0xe000_and_enabled() {
    let bar = window();
    let mut mem = Memory::new();
    mem.tx[3] = TxDesc { buffer_addr: 1, cmd_type_len: 2, olinfo_status: 3 };
    program(&regs(&bar), &mem.tx_ring(), RING);
    assert_eq!(bar.wrote32(0xE028), 0, "the queue stays off until Bus Master is on");
    assert_eq!(enable(&regs(&bar)), Ok(()));
    assert_eq!(bar.wrote32(0xE000), 0x2345_6000, "TDBAL");
    assert_eq!(bar.wrote32(0xE004), 0x0000_0001, "TDBAH");
    assert_eq!(bar.wrote32(0xE008), 512, "TDLEN, 32 x 16 bytes");
    assert_eq!(bar.wrote32(REG_TDH), 0);
    assert_eq!(bar.wrote32(REG_TDT), 0);
    let txdctl = bar.wrote32(0xE028);
    assert_eq!(txdctl, 8 | (1 << 8) | (1 << 16) | (1 << 25), "PTHRESH HTHRESH WTHRESH ENABLE");
    assert_eq!(mem.tx[3].cmd_type_len, 0, "ring cleared before the part reads it");
}
