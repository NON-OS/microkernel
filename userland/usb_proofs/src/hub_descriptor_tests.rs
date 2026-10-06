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

//! The hub descriptor and status endpoint over real devices: QEMU's
//! usb-hub, a common USB 2 hub chip and a SuperSpeed hub.

use crate::hub::descriptor::{parse_hub_descriptor, HubDescriptor};
use crate::hub::status_endpoint::{status_endpoint, StatusEndpoint};

#[test]
fn qemu_usb_hub_has_eight_ports_and_waits_the_100_ms_floor() {
    // As QEMU 11.1 hw/usb/dev-hub.c builds it for its 8 ports: bLength 10,
    // wHubCharacteristics 0x000a, bPwrOn2PwrGood 1.
    let raw = [0x0a, 0x29, 0x08, 0x0a, 0x00, 0x01, 0x00];
    let want = HubDescriptor { ports: 8, ports_named: 8, think_time: 0, power_on_ms: 100 };
    assert_eq!(parse_hub_descriptor(&raw, false), Some(want));
}

#[test]
fn a_usb2_hub_chip_gives_its_think_time_and_power_on_delay() {
    // Four ports, characteristics 0x00a9 (TT think time code 1), 50 x 2 ms.
    let raw = [0x09, 0x29, 0x04, 0xa9, 0x00, 0x32, 0x64];
    let d = parse_hub_descriptor(&raw, false).unwrap();
    assert_eq!((d.ports, d.think_time, d.power_on_ms), (4, 1, 100));
    let raw = [0x09, 0x29, 0x07, 0x60, 0x00, 0xfa, 0x00];
    let d = parse_hub_descriptor(&raw, false).unwrap();
    assert_eq!((d.ports, d.think_time, d.power_on_ms), (7, 3, 500));
}

#[test]
fn a_superspeed_hub_descriptor_is_read_only_as_one() {
    let raw = [0x0c, 0x2a, 0x04, 0x69, 0x00, 0x32, 0x00];
    let d = parse_hub_descriptor(&raw, true).unwrap();
    assert_eq!((d.ports, d.think_time), (4, 0));
    assert_eq!(parse_hub_descriptor(&raw, false), None);
    assert_eq!(parse_hub_descriptor(&[0x09, 0x29, 0x04, 0, 0, 0x32, 0], true), None);
}

#[test]
fn short_empty_or_wide_hubs_are_refused_or_capped() {
    assert_eq!(parse_hub_descriptor(&[0x09, 0x29, 0x04, 0, 0, 0x32], false), None);
    assert_eq!(parse_hub_descriptor(&[0x06, 0x29, 0x04, 0, 0, 0x32, 0], false), None);
    assert_eq!(parse_hub_descriptor(&[0x09, 0x29, 0x00, 0, 0, 0x32, 0], false), None);
    let d = parse_hub_descriptor(&[0x09, 0x29, 0x1c, 0, 0, 0x32, 0], false).unwrap();
    assert_eq!((d.ports, d.ports_named), (15, 28));
}

#[test]
fn the_status_endpoint_is_the_hub_interfaces_interrupt_in() {
    // QEMU 11.1 usb-hub: interface class 9, endpoint 0x81 interrupt, 2 byte
    // packets, bInterval 255.
    let raw = [9, 2, 25, 0, 1, 1, 0, 0xe0, 0, 9, 4, 0, 0, 1, 9, 0, 0, 0, 7, 5, 0x81, 3, 2, 0, 0xff];
    let want = StatusEndpoint { address: 0x81, max_packet: 2, interval: 0xff };
    assert_eq!(status_endpoint(&raw), Some(want));
    let mut keyboard = raw;
    keyboard[14] = 3;
    assert_eq!(status_endpoint(&keyboard), None);
    assert_eq!(status_endpoint(&raw[..20]), None);
}
