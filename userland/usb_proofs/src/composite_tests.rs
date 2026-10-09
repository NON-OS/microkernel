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

//! Devices as they come on real machines: composite keyboards, wireless
//! receivers, configurations longer than 64 bytes, and hubs.

use crate::config_blob::{config, summaries, HID, INTERRUPT};
use crate::descriptors::config::{configuration_value, is_hub, total_length, CLASS_HUB};
use crate::descriptors::hid_bindings;
use crate::descriptors::types::HidKind;

#[test]
fn a_keyboard_with_a_media_key_interface_binds_only_the_keyboard() {
    // Interface 0 boot keyboard; interface 1 consumer control, report
    // protocol with a report id, which is no tablet.
    let raw = config()
        .iface(0, HID, 1, 1)
        .hid()
        .ep(0x81, INTERRUPT, 8, 10)
        .iface(1, HID, 0, 0)
        .hid()
        .ep(0x82, INTERRUPT, 8, 10)
        .build();
    let got = summaries(&hid_bindings(&raw).expect("well formed"));
    assert_eq!(got, vec![(HidKind::Keyboard, 0, 0x81, 8)]);
}

#[test]
fn a_wireless_receiver_binds_its_keyboard_and_mouse_past_64_bytes() {
    // Keyboard, mouse and a vendor channel: 84 bytes, past the 64 the
    // driver once read, after which the parser refused the cut descriptor.
    let raw = config()
        .iface(0, HID, 1, 1)
        .hid()
        .ep(0x81, INTERRUPT, 8, 8)
        .iface(1, HID, 1, 2)
        .hid()
        .ep(0x82, INTERRUPT, 8, 2)
        .iface(2, HID, 0, 0)
        .hid()
        .ep(0x83, INTERRUPT, 32, 2)
        .build();
    assert!(raw.len() > 64, "{} bytes", raw.len());
    assert!(hid_bindings(&raw[..64]).is_err(), "a 64-byte read of it is refused");
    let got = summaries(&hid_bindings(&raw).expect("well formed"));
    assert_eq!(got, vec![(HidKind::Keyboard, 0, 0x81, 8), (HidKind::Mouse, 1, 0x82, 8)]);
    assert_eq!(total_length(&raw), Some(raw.len() as u16));
}

#[test]
fn a_device_with_no_boot_interface_is_still_a_tablet() {
    let raw = config().iface(0, HID, 0, 0).hid().ep(0x81, INTERRUPT, 8, 10).build();
    let got = summaries(&hid_bindings(&raw).expect("well formed"));
    assert_eq!(got, vec![(HidKind::Tablet, 0, 0x81, 8)]);
}

#[test]
fn the_configuration_value_is_the_descriptors_own() {
    let mut raw = config().iface(0, HID, 1, 1).ep(0x81, INTERRUPT, 8, 10).build();
    assert_eq!(configuration_value(&raw), Some(1));
    raw[5] = 2;
    assert_eq!(configuration_value(&raw), Some(2));
    assert_eq!(configuration_value(&raw[..8]), None, "no header, no value");
}

#[test]
fn a_hub_is_recognised_by_its_interface_class() {
    let hub = config().iface(0, CLASS_HUB, 0, 0).ep(0x81, INTERRUPT, 1, 12).build();
    assert!(is_hub(&hub));
    let kbd = config().iface(0, HID, 1, 1).ep(0x81, INTERRUPT, 8, 10).build();
    assert!(!is_hub(&kbd));
    // A record that runs past wTotalLength ends the walk: nothing is read
    // beyond the bytes there.
    let mut cut = hub.clone();
    cut[9] = 200;
    assert!(!is_hub(&cut));
    assert!(!is_hub(&hub[..5]));
}
