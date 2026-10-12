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

//! Which interfaces bind, by the driver's rule: a boot keyboard or mouse
//! always, any other only when its report descriptor names reports the
//! driver reads, each fitting one interrupt read.

use super::binds::{bound, one_interface};
use super::vectors::{GAMEPAD, MEDIA_KEYS, QEMU_TABLET};
use super::vectors_more::{ID_MOUSE, LONG_MOUSE, TOUCHSCREEN, UPS};
use crate::config_blob::{keyboard, HID, INTERRUPT};
use crate::descriptors::HidKind;

#[test]
fn a_gamepad_or_a_ups_binds_nothing_and_its_slot_is_given_back() {
    // The gamepad's idle report, 0x7F in every axis byte, held left, right
    // and middle down and pinned the pointer when it was read as a tablet.
    assert_eq!(bound(&one_interface(8), GAMEPAD), vec![]);
    assert_eq!(bound(&one_interface(8), UPS), vec![]);
}

#[test]
fn pointers_of_every_kind_bind() {
    for (name, desc) in [("qemu", QEMU_TABLET), ("mouse", ID_MOUSE), ("touch", TOUCHSCREEN)] {
        assert_eq!(bound(&one_interface(8), desc), vec![(HidKind::Report, 0)], "{name}");
    }
}

#[test]
fn a_keyboards_media_keys_bind_beside_its_boot_keyboard() {
    let cfg = keyboard(0).iface(1, HID, 0, 0).hid().ep(0x82, INTERRUPT, 8, 10).build();
    let want = vec![(HidKind::Keyboard, 0), (HidKind::Report, 1)];
    assert_eq!(bound(&cfg, MEDIA_KEYS), want);
    // A vendor channel there is left alone; the keyboard still binds.
    assert_eq!(bound(&cfg, GAMEPAD), vec![(HidKind::Keyboard, 0)]);
}

#[test]
fn an_interface_whose_reports_do_not_fit_one_read_is_not_bound() {
    // Nine-byte reports: the controller driver reads at most eight.
    for mps in [8u16, 9, 64, 1024] {
        assert_eq!(bound(&one_interface(mps), LONG_MOUSE), vec![], "packet {mps}");
    }
    // QEMU's six-byte reports need six-byte packets.
    for mps in 1..=16u16 {
        let want = if mps >= 6 { vec![(HidKind::Report, 0)] } else { vec![] };
        assert_eq!(bound(&one_interface(mps), QEMU_TABLET), want, "packet {mps}");
    }
}
