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

//! Records cut short or left over: the walk steps over what it cannot read
//! and never binds an endpoint to an interface it does not belong to.

use super::KBD;
use crate::config_blob::{config, keyboard, summaries, HID, INTERRUPT};
use crate::descriptors::hid_bindings;

#[test]
fn bytes_past_wtotallength_are_never_walked() {
    let keyboard_len = keyboard(0).size();
    // A whole second HID interface past the end is not bound, and a
    // zero-length record past the end is not seen.
    let raw = keyboard(0)
        .iface(1, HID, 1, 2)
        .hid()
        .ep(0x82, INTERRUPT, 4, 10)
        .raw(&[0, 0])
        .build_with_total(keyboard_len as u16);
    assert_eq!(summaries(&hid_bindings(&raw).expect("first part")), vec![KBD]);
}

#[test]
fn a_single_byte_left_after_the_last_record_ends_the_walk() {
    for b in 0..=u8::MAX {
        let raw = keyboard(0).raw(&[b]).build();
        assert_eq!(summaries(&hid_bindings(&raw).expect("one stray byte")), vec![KBD]);
    }
}

#[test]
fn a_truncated_endpoint_is_stepped_over_and_never_bound() {
    for len in 2u8..7 {
        let mut rec = vec![len, 0x05];
        rec.resize(len as usize, 0x81);
        let raw = config().iface(0, HID, 1, 1).raw(&rec).build();
        assert_eq!(hid_bindings(&raw).map(|b| b.len()), Ok(0), "bLength {len}");
    }
}

#[test]
fn a_truncated_interface_ends_the_interface_before_it() {
    // A keyboard, then an interface record cut short, then an interrupt IN
    // endpoint. The endpoint belongs to the cut record, which names no
    // class, so it is not bound as a second keyboard endpoint.
    for len in 2u8..9 {
        let mut rec = vec![len, 0x04];
        rec.resize(len as usize, HID);
        let raw = keyboard(0).raw(&rec).ep(0x82, INTERRUPT, 8, 10).build();
        let got = summaries(&hid_bindings(&raw).expect("walked"));
        assert_eq!(got, vec![KBD], "interface bLength {len}");
    }
}
