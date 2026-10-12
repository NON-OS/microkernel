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

//! Binding: how much one look at the event endpoint arms, and which
//! configuration is taken, against descriptors laid out by hand (USB 2.0,
//! 9.6.3, 9.6.5, 9.6.6; class triple Vol 4, Part B, 3).

use crate::usb::choose::{choose, Verdict};
use crate::usb::limit::{read_limit, INTERRUPT_MAX};

const fn config(value: u8) -> [u8; 9] {
    [9, 2, 0, 0, 2, value, 0, 0x80, 50]
}
const BT: [u8; 9] = [9, 4, 0, 0, 3, 0xe0, 0x01, 0x01, 0];
const HID: [u8; 9] = [9, 4, 0, 0, 1, 0x03, 0x01, 0x01, 0];
const fn events(max_packet: u16) -> [u8; 7] {
    let [lo, hi] = max_packet.to_le_bytes();
    [7, 5, 0x81, 3, lo, hi, 1]
}
const ACL_IN: [u8; 7] = [7, 5, 0x82, 2, 64, 0, 0];
const ACL_OUT: [u8; 7] = [7, 5, 0x02, 2, 64, 0, 0];

fn controller(value: u8, max_packet: u16) -> Vec<u8> {
    [&config(value)[..], &BT, &events(max_packet), &ACL_IN, &ACL_OUT].concat()
}

#[test]
fn a_read_arms_the_whole_packet_and_never_more_than_64() {
    assert_eq!(read_limit(16), Some(16));
    assert_eq!(read_limit(64), Some(INTERRUPT_MAX));
    assert_eq!(read_limit(65), None, "over B-43's cap: a read of 65 is E_INVAL");
    assert_eq!(read_limit(0), None);
    // Bits 12:11 are extra transactions per microframe, not packet length.
    assert_eq!(read_limit(0x0810), Some(16));
    assert_eq!(read_limit(0x1840), Some(64));
}

#[test]
fn the_bluetooth_configuration_is_taken_with_its_value() {
    let hid_only = [&config(1)[..], &HID, &events(8)].concat();
    let Verdict::Ours(c) = choose(&[hid_only, controller(2, 16)]) else { panic!("not taken") };
    assert_eq!((c.value, c.event_read, c.iface.events.max_packet), (2, 16, 16));
    assert_eq!((c.iface.acl_in.address, c.iface.acl_out.address), (0x82, 0x02));
}

#[test]
fn other_devices_and_unreadable_controllers_are_told_apart() {
    let hid_only = [&config(1)[..], &HID, &events(8)].concat();
    assert_eq!(choose(&[hid_only]), Verdict::NotOurs);
    assert_eq!(choose(&[]), Verdict::NotOurs);
    let wide = Verdict::Unusable("event endpoint packet over 64 octets");
    assert_eq!(choose(&[controller(1, 128)]), wide);
    // An 8-octet header whose walk still finds the interface (bLength must
    // be 9, USB 2.0, 9.6.3), and descriptors that are not a configuration.
    let mut short = controller(1, 16);
    short[0] = 8;
    short.remove(8);
    let mut not_config = controller(1, 16);
    not_config[1] = 4;
    assert_eq!(choose(&[short, not_config, vec![9, 2]]), Verdict::NotOurs);
}
