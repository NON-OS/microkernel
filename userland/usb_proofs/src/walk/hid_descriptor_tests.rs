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

//! The HID class descriptor after an interface gives the length of its
//! report descriptor, if long enough and naming one first; the length
//! belongs to the interface it follows and no other.

use crate::config_blob::{config, HID, INTERRUPT};
use crate::descriptors::hid_bindings;

/// A HID class descriptor of `len` bytes naming `kind` first, `size` long.
fn hid_desc(len: u8, kind: u8, size: u16) -> Vec<u8> {
    let [lo, hi] = size.to_le_bytes();
    let mut rec = vec![len, 0x21, 0x11, 0x01, 0, 1, kind, lo, hi];
    rec.resize(usize::from(len), 0);
    rec
}

fn report_len(rec: &[u8]) -> Vec<u16> {
    let raw = config().iface(0, HID, 0, 0).raw(rec).ep(0x81, INTERRUPT, 8, 10).build();
    hid_bindings(&raw).expect("well formed").iter().map(|b| b.report_len).collect()
}

#[test]
fn the_report_descriptor_length_is_the_hid_descriptors_own() {
    for size in [0u16, 1, 74, 512, 513, u16::MAX] {
        assert_eq!(report_len(&hid_desc(9, 0x22, size)), vec![size], "{size}");
    }
    // A longer record, as one listing more descriptors is, reads the same.
    assert_eq!(report_len(&hid_desc(12, 0x22, 63)), vec![63]);
}

#[test]
fn a_hid_descriptor_cut_short_or_naming_another_first_gives_no_length() {
    for len in 2u8..9 {
        assert_eq!(report_len(&hid_desc(len, 0x22, 63)), vec![0], "bLength {len}");
    }
    for kind in (0..=u8::MAX).filter(|&k| k != 0x22) {
        assert_eq!(report_len(&hid_desc(9, kind, 63)), vec![0], "type {kind:#x}");
    }
}

#[test]
fn a_length_belongs_to_the_interface_it_follows() {
    // Interface 0 with a length, interface 1 with none of its own.
    let raw = config()
        .iface(0, HID, 0, 0)
        .raw(&hid_desc(9, 0x22, 99))
        .ep(0x81, INTERRUPT, 8, 10)
        .iface(1, HID, 0, 0)
        .ep(0x82, INTERRUPT, 8, 10)
        .build();
    let lens: Vec<u16> = hid_bindings(&raw).expect("ok").iter().map(|b| b.report_len).collect();
    assert_eq!(lens, vec![99, 0]);
    // One before any interface belongs to none.
    assert_eq!(report_len_before_interface(), vec![0]);
}

fn report_len_before_interface() -> Vec<u16> {
    let raw = config().raw(&hid_desc(9, 0x22, 99)).iface(0, HID, 0, 0).ep(0x81, INTERRUPT, 8, 10);
    hid_bindings(&raw.build()).expect("ok").iter().map(|b| b.report_len).collect()
}
