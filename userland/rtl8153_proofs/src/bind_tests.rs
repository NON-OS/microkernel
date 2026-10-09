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

//! Binding the model in Linux's order, and the state the bring-up leaves.

use nonos_usbnet::mock::Call;
use nonos_usbnet::{Bind, Nic, Setup};

use crate::chip::{bound, found, PHY, PLA, RTL8153_IDS, TEST_MAC, USB};
use crate::r8153::{bind, Version};

#[test]
fn the_vendor_configuration_is_chosen_after_the_version_is_read() {
    let (bus, _, found) = found(0x5c10, RTL8153_IDS);
    let Bind::Ours(nic) = bind(bus.clone(), &found) else { panic!("not bound") };
    assert_eq!((nic.mac(), nic.version()), (TEST_MAC, Version::V04));
    let calls = bus.0.borrow().calls.clone();
    assert_eq!(calls[0], Call::In(Setup::new(0xc0, 0x05, 0xe610, 0x0100), 4));
    let Call::Configure(p) = calls[1] else { panic!("pipes not first") };
    assert_eq!((p.bulk_in, p.bulk_out, p.max_packet_in, p.burst_out), (0x81, 0x02, 1024, 3));
    assert_eq!(calls[2], Call::Out(Setup::set_configuration(2), vec![]));
}

#[test]
fn the_mac_goes_into_pla_idr_as_one_whole_dword_and_one_half() {
    let (bus, _, found) = found(0x5c10, RTL8153_IDS);
    let _ = bind(bus.clone(), &found);
    let calls = bus.0.borrow().calls.clone();
    let at = calls.iter().position(|c| matches!(c, Call::Out(s, _) if s.value == 0xc000)).unwrap();
    let w = |v, i, d: &[u8]| Call::Out(Setup::new(0x40, 0x05, v, i), d.to_vec());
    assert_eq!(calls[at - 1], w(0xe81c, 0x0111, &[0xc0, 0, 0, 0]));
    assert_eq!(calls[at], w(0xc000, 0x01ff, &TEST_MAC[..4]));
    assert_eq!(calls[at + 1], w(0xc004, 0x0133, &[0x53, 0x01, 0, 0]));
    assert_eq!(calls[at + 2], w(0xe81c, 0x0111, &[0, 0, 0, 0]));
}

#[test]
fn an_rtl8153_is_left_ready_for_traffic_but_not_yet_passing_it() {
    let (nic, _, chip) = bound(0x5c10);
    let r = &chip.borrow().regs;
    assert!(!nic.link_up());
    assert_eq!(r.word(USB, 0xd406) & 0x0090, 0x0010, "RX_AGG_DISABLE on, RX_ZERO_EN off");
    assert_eq!(r.read(PLA, 0xc000, 6), TEST_MAC);
    assert_eq!(r.dword(PLA, 0xc010) & 0x0f, 0, "nothing accepted yet");
    assert_eq!(r.byte(PLA, 0xe813) & 0x0c, 0, "TX and RX off until the link");
    assert_eq!(r.word(PLA, 0xe85a) & 0x0008, 0x0008, "RX gate closed");
    assert_eq!(r.byte(PLA, 0xe84f) & 0x80, 0, "out of OOB mode");
    assert_eq!(r.word(PLA, 0xe8de) & 0xc000, 0x8000, "MCU hold off, link list rebuilt");
    assert_eq!(r.word(PLA, 0xe854) & 0x0040, 0, "VLAN tags left in the frame");
    assert_eq!((r.word(PLA, 0xc016), r.byte(PLA, 0xe615)), (1522, 192));
    assert_eq!(r.dword(PLA, 0xc0a0), 0x0008_0002);
    assert_eq!(r.dword(PLA, 0xe618), 0x0100_0008);
    assert_eq!(r.word(PHY, 0xa400), 0x1200, "autoneg on and restarted, out of power down");
    assert_eq!(r.word(PHY, 0xa408), 0x0de1, "10/100 and pause advertised");
    assert_eq!(r.word(PHY, 0xa412), 0x0200, "1000 full only");
    assert_eq!(r.word(PHY, 0xa430) & 0x0004, 0, "ALDPS off");
    assert_eq!(r.word(PHY, 0xa436), 0x8082, "SRAM tuning ended at the 10M amplitude");
    assert_eq!(r.word(PHY, 0xa438), 0x0208);
    assert_eq!(r.word(PHY, 0xbc06), 0x01c0, "ADC config on RTL_VER_04");
}
