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

//! The start before the enable, against Linux rtl_hw_start,
//! rtl_hw_start_8169, rtl_hw_start_8168 and rtl8169_set_magic_reg.

use std::cell::Cell;

use super::model::window;
use crate::chip::MacVersion;
use crate::constants::regs::REG_CFG9346;
use crate::hw::start;
use crate::regs::Regs;

#[test]
fn the_rings_are_programmed_unlocked_and_the_lock_is_put_back() {
    let bar = window();
    let seen = Cell::new(0u8);
    start(&Regs::new(bar.base()), MacVersion(46), || seen.set(bar.wrote8(REG_CFG9346)));
    assert_eq!(seen.get(), 0xC0, "Cfg9346 unlocked while the rings are written");
    assert_eq!(bar.wrote8(REG_CFG9346), 0x00, "and locked after");
}

#[test]
fn an_8168_gets_max_tx_packet_size_by_generation_and_no_coalescing() {
    for (v, size) in [(17u8, 0x3F), (25, 0x3F), (34, 0x27), (46, 0x27), (39, 0x3F)] {
        let bar = window();
        bar.present16(0xE2, 0xFFFF);
        start(&Regs::new(bar.base()), MacVersion(v), || {});
        assert_eq!(bar.wrote8(0xEC), size, "VER_{v} MaxTxPacketSize");
        assert_eq!(bar.wrote16(0xE2), 0, "VER_{v} IntrMitigate");
    }
}

#[test]
fn an_8169_gets_no_early_tx_and_the_8169sc_its_magic_register() {
    let bar = window();
    start(&Regs::new(bar.base()), MacVersion(2), || {});
    assert_eq!(bar.wrote8(0xEC), 0x3F, "EarlyTxThres NoEarlyTx");
    assert_eq!(bar.wrote32(0x7C), 0, "no magic on the 8169s");
    let bar = window();
    bar.present8(0x53, 0x01);
    start(&Regs::new(bar.base()), MacVersion(5), || {});
    assert_eq!(bar.wrote32(0x7C), 0x000F_FFFF, "VER_05 on a 66 MHz bus");
    let bar = window();
    start(&Regs::new(bar.base()), MacVersion(6), || {});
    assert_eq!(bar.wrote32(0x7C), 0x00FF_FF00, "VER_06 on a 33 MHz bus");
}

#[test]
fn an_8125_keeps_0xec_and_0xe2_for_its_own_start() {
    let bar = window();
    bar.present16(0xE2, 0x1234);
    bar.present8(0xEC, 0x55);
    start(&Regs::new(bar.base()), MacVersion(63), || {});
    assert_eq!((bar.wrote8(0xEC), bar.wrote16(0xE2)), (0x55, 0x1234));
}
