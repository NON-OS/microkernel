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

//! The send and receive paths on an 8125 and an 8168: each reaches the
//! doorbell and the status register its version has.

use super::memory::Memory;
use super::model::window;
use super::regmap_tests::RTL8125B;
use crate::rx::recv_one;
use crate::tx::send;

#[test]
fn an_8125_send_rings_txpoll_8125_and_leaves_its_interrupt_mask_alone() {
    let bar = window();
    let mut mem = Memory::new();
    let mut d = mem.driver_with(&bar, RTL8125B);
    send(&mut d, &[0u8; 60]);
    assert_eq!(bar.wrote16(0x90), 0x0001, "TxPoll_8125 bit 0");
    assert_eq!(bar.wrote32(0x38), 0, "IntrMask_8125 is not written by a send");
}

#[test]
fn an_8168_send_rings_tppoll_npq() {
    let bar = window();
    let mut mem = Memory::new();
    let mut d = mem.driver(&bar);
    send(&mut d, &[0u8; 60]);
    assert_eq!(bar.wrote8(0x38), 0x40);
    assert_eq!(bar.wrote16(0x90), 0, "0x90 is not a doorbell on an 8168");
}

#[test]
fn an_8125_receive_error_is_read_from_intrstatus_8125() {
    let bar = window();
    bar.present32(0x3C, 0x0000_0002);
    let mut mem = Memory::new();
    let mut d = mem.driver_with(&bar, RTL8125B);
    let mut out = [0u8; 2048];
    assert_eq!(recv_one(&mut d, &mut out), Err("rtl8169 rx interrupt error"));
    assert_eq!(bar.wrote32(0x3C), 0x0000_0002, "the error bit is written back to clear it");
}
