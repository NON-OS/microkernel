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

//! Traffic started when the link comes up, as set_carrier runs
//! rtl8153_enable and _rtl8152_set_rx_mode.

use nonos_usbnet::Nic;

use crate::chip::{bound, PLA, USB};

const PHYSTATUS: u16 = 0xe908;

#[test]
fn a_link_up_starts_tx_and_rx_with_broadcast_multicast_and_own_address() {
    let (mut nic, _, chip) = bound(0x5c10);
    // LINK_STATUS, 1000 Mb/s, full duplex.
    chip.borrow_mut().regs.put(PLA, PHYSTATUS, &0x0013u16.to_le_bytes());
    assert_eq!(nic.recv(&mut [0u8; 1514]), Ok(None));
    assert!(nic.link_up());
    let r = &chip.borrow().regs;
    assert_eq!(r.byte(PLA, 0xe813) & 0x0c, 0x0c, "CR_RE and CR_TE");
    assert_eq!(r.dword(PLA, 0xc010) & 0x0f, 0x0e, "RCR_AB, RCR_AM, RCR_APM, not RCR_AAP");
    assert_eq!(r.read(PLA, 0xcd00, 8), [0xff; 8], "all multicast hashes");
    assert_eq!(r.word(PLA, 0xe85a) & 0x0008, 0, "RX gate open");
    assert_eq!(r.word(PLA, 0xc0b4) & 0x0001, 0x0001, "packet filter back on");
    assert_eq!(r.word(PLA, 0xe612) & 0x0308, 0x0300, "96 ns gap at full duplex");
    assert_eq!(r.word(PLA, 0xe612) & 0x7cf0, 0x5c10, "version bits kept");
}

#[test]
fn a_half_duplex_link_gets_the_long_gap() {
    let (mut nic, _, chip) = bound(0x5c30);
    // LINK_STATUS, 100 Mb/s, half duplex.
    chip.borrow_mut().regs.put(PLA, PHYSTATUS, &0x000au16.to_le_bytes());
    nic.recv(&mut [0u8; 1514]).unwrap();
    assert_eq!(chip.borrow().regs.word(PLA, 0xe612) & 0x0308, 0x0200);
}

#[test]
fn rtl_ver_09_restarts_its_flow_control_task_and_updates_the_rx_dma_owner() {
    let (mut nic, _, chip) = bound(0x6010);
    chip.borrow_mut().regs.put(PLA, PHYSTATUS, &0x0013u16.to_le_bytes());
    nic.recv(&mut [0u8; 1514]).unwrap();
    let r = &chip.borrow().regs;
    assert_eq!(r.word(USB, 0xd4e8) & 0x0002, 0x0002, "FC_PATCH_TASK set again");
    assert_eq!(r.byte(USB, 0xd437), 0x03, "OWN_UPDATE | OWN_CLEAR");
    assert_eq!(r.byte(PLA, 0xe813) & 0x0c, 0x0c);
}
