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

//! A driver's button and wheel, as wl_pointer reads them.

use crate::pointer_codes::{button_code, wheel_step};

#[test]
fn buttons_from_one_are_btn_left_onward() {
    let linux = [(1, 0x110), (2, 0x111), (3, 0x112), (4, 0x113), (5, 0x114)];
    linux.iter().for_each(|(b, code)| assert_eq!(button_code(*b), Some(*code), "button {b}"));
    for b in [0, 6, 0x110, u32::MAX] {
        assert_eq!(button_code(b), None, "button {b}");
    }
}

#[test]
fn a_notch_away_scrolls_up_by_fifteen_units() {
    // Away from the person is +1 on the ring, and up is negative to a client.
    assert_eq!(wheel_step(1), Some((-1, (-15i32 * 256) as u32)));
    assert_eq!(wheel_step(-2), Some((2, 30 * 256)));
    assert_eq!(wheel_step(0), None);
}

#[test]
fn a_step_too_large_to_carry_is_dropped() {
    assert_eq!(wheel_step(i32::MIN), None);
    assert_eq!(wheel_step(i32::MAX), None);
    assert!(wheel_step(500_000).is_some());
}
