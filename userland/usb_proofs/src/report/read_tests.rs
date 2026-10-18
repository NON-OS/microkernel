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

//! How a report descriptor is read before a binding is driven by it: whole,
//! in one read of at most 512 bytes, as long as its HID descriptor says. A
//! longer one was walked cut at 512 bytes, and a cut between two collections
//! parsed, leaving the reports past it unread; one given short was walked as
//! far as it went. Both are now refused, and the interface left unbound.

use super::binds::one_interface;
use super::vectors::{GAMEPAD, QEMU_TABLET};
use crate::config_blob::keyboard;
use crate::descriptors::{hid_bindings, HidBinding};
use crate::hid::Device;
use crate::orchestrator::device_for::device_for;
use crate::xhci::report_desc::{reads, serve};

/// The first binding of `cfg`, its report descriptor `report_len` long.
fn binding(cfg: &[u8], report_len: u16) -> HidBinding {
    HidBinding { report_len, ..hid_bindings(cfg).expect("well formed")[0] }
}

/// QEMU's tablet padded with empty items to `len` bytes.
fn padded(len: usize) -> Vec<u8> {
    let mut d = QEMU_TABLET.to_vec();
    d.resize(len, 0);
    d
}

#[test]
fn a_descriptor_of_one_read_binds_and_a_longer_one_is_refused_unread() {
    serve(&padded(512));
    assert!(device_for(7, 1, binding(&one_interface(8), 512)).is_some());
    assert_eq!(reads(), 1);
    // The tablet's collection closes inside the first 512 bytes; a gamepad's
    // follows past them.
    let long = [padded(512), GAMEPAD.to_vec()].concat();
    serve(&long);
    assert!(device_for(7, 1, binding(&one_interface(8), long.len() as u16)).is_none());
    assert_eq!(reads(), 0, "refused before it is read");
}

#[test]
fn a_descriptor_given_shorter_than_its_length_is_refused() {
    serve(QEMU_TABLET);
    let says = QEMU_TABLET.len() as u16;
    assert!(device_for(7, 1, binding(&one_interface(8), says)).is_some());
    for more in [1, 26, 438] {
        assert!(device_for(7, 1, binding(&one_interface(8), says + more)).is_none(), "+{more}");
    }
}

#[test]
fn no_descriptor_length_is_no_binding_and_a_boot_keyboard_needs_none() {
    serve(QEMU_TABLET);
    assert!(device_for(7, 1, binding(&one_interface(8), 0)).is_none());
    let boot = device_for(7, 1, binding(&keyboard(0).build(), 0));
    assert!(matches!(boot, Some((_, Device::Keyboard(_)))));
    assert_eq!(reads(), 0, "neither is read");
}
