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

//! Finding the Bluetooth interface in configuration descriptors laid out
//! by hand (USB 2.0, 9.6.3, 9.6.5, 9.6.6; class triple Vol 4, Part B, 3).

use nonos_bt_hci::usb::{find, Endpoint};
use nonos_bt_hci::HciError;
use nonos_bt_wire::WireError;

const CONFIG: [u8; 9] = [9, 2, 0, 0, 2, 1, 0, 0x80, 50];
const fn iface(n: u8, alt: u8, class: [u8; 3]) -> [u8; 9] {
    [9, 4, n, alt, 3, class[0], class[1], class[2], 0]
}
const EVENTS: [u8; 7] = [7, 5, 0x81, 3, 16, 0, 1];
const ACL_IN: [u8; 7] = [7, 5, 0x82, 2, 64, 0, 0];
const ACL_OUT: [u8; 7] = [7, 5, 0x02, 2, 64, 0, 0];
const BT: [u8; 3] = [0xe0, 0x01, 0x01];

fn join(parts: &[&[u8]]) -> Vec<u8> {
    parts.concat()
}

#[test]
fn finds_interface_zero_and_ignores_the_voice_interface() {
    let iso = [7, 5, 0x83, 1, 9, 0, 1];
    let c = join(&[&CONFIG, &iface(0, 0, BT), &EVENTS, &ACL_IN, &ACL_OUT, &iface(1, 0, BT), &iso]);
    let bt = find(&c).unwrap();
    assert_eq!(bt.number, 0);
    assert_eq!(bt.events, Endpoint { address: 0x81, max_packet: 16, interval: 1 });
    assert_eq!(bt.acl_in, Endpoint { address: 0x82, max_packet: 64, interval: 0 });
    assert_eq!(bt.acl_out, Endpoint { address: 0x02, max_packet: 64, interval: 0 });
}

#[test]
fn skips_another_class_and_takes_only_alternate_setting_zero() {
    let vendor = iface(0, 0, [0xff, 0xff, 0xff]);
    let c =
        join(&[&CONFIG, &vendor, &ACL_IN, &ACL_OUT, &iface(1, 0, BT), &EVENTS, &ACL_IN, &ACL_OUT]);
    assert_eq!(find(&c).unwrap().number, 1);
    let only_alt1 =
        join(&[&CONFIG, &iface(0, 0, BT), &iface(0, 1, BT), &EVENTS, &ACL_IN, &ACL_OUT]);
    assert_eq!(find(&only_alt1), Err(HciError::NotBluetooth));
}

#[test]
fn refuses_missing_endpoints_and_broken_lengths() {
    let no_out = join(&[&CONFIG, &iface(0, 0, BT), &EVENTS, &ACL_IN]);
    assert_eq!(find(&no_out), Err(HciError::NotBluetooth));
    let zero = join(&[&CONFIG, &[0, 4]]);
    assert_eq!(find(&zero), Err(HciError::Wire(WireError::Length)));
    let past_end = join(&[&CONFIG, &iface(0, 0, BT), &[7, 5, 0x81]]);
    assert_eq!(find(&past_end), Err(HciError::Wire(WireError::Length)));
}
