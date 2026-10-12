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

//! Only a HID class interface's interrupt IN endpoint binds, and a boot
//! interface only as the keyboard or mouse it says it is; a buffer of
//! padding binds nothing.

use crate::config_blob::{config, HID, INTERRUPT};
use crate::descriptors::hid_bindings;
use crate::descriptors::types::HidKind;

#[test]
fn an_endpoint_before_any_interface_is_not_bound() {
    let raw = config().ep(0x81, INTERRUPT, 8, 10).iface(0, HID, 1, 1).build();
    assert_eq!(hid_bindings(&raw).map(|b| b.len()), Ok(0));
}

#[test]
fn only_a_hid_class_interface_binds() {
    for class in (0..=u8::MAX).filter(|&c| c != HID) {
        let raw = config().iface(0, class, 1, 1).ep(0x81, INTERRUPT, 8, 10).build();
        assert_eq!(hid_bindings(&raw).map(|b| b.len()), Ok(0), "class {class:#x}");
    }
}

#[test]
fn only_an_interrupt_in_endpoint_binds() {
    for attributes in 0..=u8::MAX {
        for address in [0x01u8, 0x81] {
            let raw = config().iface(0, HID, 1, 1).ep(address, attributes, 8, 10).build();
            let bound = hid_bindings(&raw).expect("well formed").len();
            let interrupt_in = address & 0x80 != 0 && attributes & 0x03 == INTERRUPT;
            assert_eq!(bound, usize::from(interrupt_in), "{address:#x} {attributes:#x}");
        }
    }
}

#[test]
fn a_boot_interface_of_no_known_protocol_is_not_bound() {
    for protocol in 0..=u8::MAX {
        let raw = config().iface(0, HID, 1, protocol).ep(0x81, INTERRUPT, 8, 10).build();
        let kinds: Vec<_> = hid_bindings(&raw).expect("ok").iter().map(|b| b.kind).collect();
        let want = match protocol {
            1 => vec![HidKind::Keyboard],
            2 => vec![HidKind::Mouse],
            _ => vec![],
        };
        assert_eq!(kinds, want, "protocol {protocol}");
    }
}

#[test]
fn the_largest_wtotallength_walks_a_full_buffer_of_padding() {
    // 0xFFFF bytes of two-byte records: the walk ends at the buffer's end.
    let mut d = vec![9, 0x02, 0xff, 0xff, 1, 1, 0, 0xa0, 50];
    while d.len() + 2 <= u16::MAX as usize {
        d.extend_from_slice(&[2, 0x24]);
    }
    d.resize(u16::MAX as usize, 0);
    assert_eq!(hid_bindings(&d).map(|b| b.len()), Ok(0));
}
