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

//! Boot mouse buttons: only the five defined button bits ever become
//! buttons, a report that changes nothing is no event, and each button
//! event says which button went which way.

use super::motion_tests::fields;
use crate::hid::button_changes::button_changes;
use crate::hid::mouse_report::MOUSE_BUTTONS;

/// Every button event a pair of button bytes makes under `mask`.
pub fn buttons(previous: u8, current: u8, mask: u8) -> Vec<(u32, bool)> {
    let mut out = Vec::new();
    button_changes(previous, current, mask, |b, down| out.push((b, down)));
    out
}

#[test]
fn only_the_five_button_bits_are_buttons() {
    for b in 0..=u8::MAX {
        let got = fields(&[b, 0, 0], 0);
        let held = b & 0x1f;
        if held == 0 {
            assert_eq!(got, None, "byte {b:#x} with nothing held is no event");
        } else {
            assert_eq!(got, Some((0, 0, 0, held)), "byte {b:#x}");
        }
    }
}

#[test]
fn a_report_that_changes_nothing_is_no_event() {
    for held in 0..=MOUSE_BUTTONS {
        assert_eq!(fields(&[held, 0, 0, 0], held), None);
        // Padding bits alone are no change.
        assert_eq!(fields(&[held | 0xe0, 0, 0], held), None);
    }
}

#[test]
fn button_events_name_only_defined_buttons_and_say_which_way_each_went() {
    // The boot mouse's five buttons, and a three-button mask as QEMU's
    // tablet once had.
    for (mask, top) in [(MOUSE_BUTTONS, 5u32), (0x07, 3)] {
        for previous in 0..=u8::MAX {
            for current in 0..=u8::MAX {
                let got = buttons(previous, current, mask);
                let want: Vec<(u32, bool)> = (0..8u32)
                    .filter(|&bit| bit < top && ((previous ^ current) >> bit) & 1 == 1)
                    .map(|bit| (bit + 1, (current >> bit) & 1 == 1))
                    .collect();
                assert_eq!(got, want, "{previous:#x} to {current:#x} under {mask:#x}");
            }
        }
    }
}
