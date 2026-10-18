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

//! The configuration descriptor walk over the lengths the device chose:
//! every record's bLength and the header's wTotalLength. The walk must end
//! and read nothing past wTotalLength or the buffer.

use super::KBD;
use crate::config_blob::{keyboard, summaries, HID, INTERRUPT};
use crate::descriptors::hid_bindings;
use crate::descriptors::types::HidKind;

#[test]
fn a_keyboard_and_a_mouse_bind_in_descriptor_order() {
    let raw = keyboard(0).iface(1, HID, 1, 2).hid().ep(0x82, INTERRUPT, 4, 10).build();
    let got = summaries(&hid_bindings(&raw).expect("well formed"));
    assert_eq!(got, vec![KBD, (HidKind::Mouse, 1, 0x82, 4)]);
}

#[test]
fn a_blength_of_zero_or_one_is_refused_rather_than_looped_on() {
    for len in [0u8, 1] {
        let raw = keyboard(0).raw(&[len, 0x24]).build();
        assert_eq!(hid_bindings(&raw).map(|b| b.len()), Err(()), "bLength {len}");
    }
}

#[test]
fn a_blength_past_the_bytes_left_is_refused() {
    let base = keyboard(0);
    let left = 4u8;
    // A record that claims one byte more than wTotalLength leaves it.
    let raw = base.raw(&[left + 1, 0x24, 0, 0]).build();
    assert_eq!(hid_bindings(&raw).map(|b| b.len()), Err(()));
    // A record whose bLength is exactly the bytes left is walked.
    let raw = keyboard(0).raw(&[left, 0x24, 0, 0]).build();
    assert_eq!(summaries(&hid_bindings(&raw).expect("fits")), vec![KBD]);
}

#[test]
fn every_wtotallength_against_one_buffer_is_decided_without_reading_past_it() {
    let blob = keyboard(0);
    let len = blob.size();
    let raw = blob.build();
    for total in 0..=u16::MAX {
        let mut d = raw.clone();
        d[2..4].copy_from_slice(&total.to_le_bytes());
        let got = hid_bindings(&d);
        let total = total as usize;
        if total < 9 || total > len {
            assert!(got.is_err(), "wTotalLength {total} of a {len} byte buffer");
        } else if total == len {
            assert_eq!(summaries(&got.expect("whole")), vec![KBD]);
        } else {
            // Shorter than the buffer: the walk stops at wTotalLength, and
            // the endpoint cut by it is never read.
            assert!(got.map_or(true, |b| b.is_empty()), "wTotalLength {total}");
        }
    }
}
