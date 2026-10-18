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

//! Composite devices as they come on real machines: a keyboard with a media
//! key interface, a wireless receiver past 64 bytes, a device with no boot
//! interface. Every HID interface is offered; one in report protocol is
//! bound only if its report descriptor names reports the driver reads.

use crate::config_blob::{config, summaries, HID, INTERRUPT};
use crate::descriptors::config::total_length;
use crate::descriptors::hid_bindings;
use crate::descriptors::types::HidKind;

#[test]
fn a_keyboard_with_a_media_key_interface_offers_both() {
    // Interface 0 boot keyboard; interface 1 consumer control in report
    // protocol, bound only if its report descriptor names keys the driver
    // reads (report::bind_tests). It used to be dropped unread.
    let raw = config()
        .iface(0, HID, 1, 1)
        .hid()
        .ep(0x81, INTERRUPT, 8, 10)
        .iface(1, HID, 0, 0)
        .hid()
        .ep(0x82, INTERRUPT, 8, 10)
        .build();
    let got = summaries(&hid_bindings(&raw).expect("well formed"));
    assert_eq!(got, vec![(HidKind::Keyboard, 0, 0x81, 8), (HidKind::Report, 1, 0x82, 8)]);
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
    let vendor = (HidKind::Report, 2, 0x83, 32);
    assert_eq!(got, vec![(HidKind::Keyboard, 0, 0x81, 8), (HidKind::Mouse, 1, 0x82, 8), vendor]);
    assert_eq!(total_length(&raw), Some(raw.len() as u16));
}

#[test]
fn a_device_with_no_boot_interface_is_read_by_its_report_descriptor() {
    let raw = config().iface(0, HID, 0, 0).hid().ep(0x81, INTERRUPT, 8, 10).build();
    let got = hid_bindings(&raw).expect("well formed");
    assert_eq!(summaries(&got), vec![(HidKind::Report, 0, 0x81, 8)]);
    assert_eq!(got[0].report_len, 63, "the length its HID class descriptor gives");
}
