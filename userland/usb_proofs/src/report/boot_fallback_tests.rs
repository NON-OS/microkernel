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

//! A boot keyboard that STALLs SET_PROTOCOL(boot) stays in report protocol,
//! where its reports may start with a report ID. Read in the boot layout,
//! that ID byte was taken for the modifier keys: report ID 1 held Left Ctrl
//! down under every key. It is read through its report descriptor instead,
//! and left unbound when that names nothing this driver reads.

use super::vectors::GAMEPAD;
use super::vectors_more::UPS;
use crate::config_blob::keyboard;
use crate::descriptors::{hid_bindings, HidBinding, HidKind};
use crate::hid::Device;
use crate::orchestrator::device_for::device_for;
use crate::xhci::report_desc::{serve, stall_boot_protocol};
use nonos_libc::{reset, take_posted};

/// A keyboard under report ID 1: eight modifier bits, six key slots.
const ID_KEYBOARD: &[u8] = &[
    0x05, 0x01, 0x09, 0x06, 0xa1, 0x01, 0x85, 0x01, 0x05, 0x07, 0x19, 0xe0, 0x29, 0xe7, 0x15, 0x00,
    0x25, 0x01, 0x75, 0x01, 0x95, 0x08, 0x81, 0x02, 0x95, 0x06, 0x75, 0x08, 0x25, 0x65, 0x19, 0x00,
    0x29, 0x65, 0x81, 0x00, 0xc0,
];

fn boot_keyboard(report_len: usize) -> HidBinding {
    let b = hid_bindings(&keyboard(0).build()).expect("well formed")[0];
    HidBinding { report_len: report_len as u16, ..b }
}

#[test]
fn a_keyboard_refusing_boot_protocol_is_read_by_its_report_descriptor() {
    reset();
    serve(ID_KEYBOARD);
    stall_boot_protocol();
    let Some((bound, mut dev)) = device_for(7, 1, boot_keyboard(ID_KEYBOARD.len())) else {
        panic!("not bound");
    };
    assert_eq!(bound.kind, HidKind::Report);
    // Report ID 1, no modifier, A: in the boot layout, Ctrl+A.
    dev.feed(&[0x01, 0x00, 0x04, 0, 0, 0, 0, 0]);
    let flags: Vec<(u32, u16)> = take_posted().iter().map(|e| (e.code, e.flags)).collect();
    assert_eq!(flags, vec![(u32::from(b'a'), 0)], "no Ctrl from the report ID");
    assert!(matches!(dev, Device::Report(_)));
}

#[test]
fn a_keyboard_refusing_boot_protocol_with_no_layout_is_left_unbound() {
    serve(UPS);
    stall_boot_protocol();
    assert!(device_for(7, 1, boot_keyboard(UPS.len())).is_none());
}

#[test]
fn a_boot_interface_refusing_boot_protocol_that_lays_out_a_gamepad_is_a_pad() {
    serve(GAMEPAD);
    stall_boot_protocol();
    let (bound, dev) = device_for(7, 1, boot_keyboard(GAMEPAD.len())).expect("a pad");
    assert_eq!(bound.kind, HidKind::Gamepad);
    assert!(matches!(dev, Device::Pad(_)));
}
