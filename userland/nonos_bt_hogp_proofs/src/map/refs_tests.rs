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

//! Report References that do not lead to a keyboard: wrong lengths, IDs
//! the map has not, or only Consumer keys. Each Report is passed over,
//! and with none left that types, the keyboard stays in boot mode.

use super::fallback_tests::boot;
use super::maps::KEYBOARD;
use crate::sim::report_devices::*;
use crate::sim::Run;

#[test]
fn references_of_the_wrong_length_are_passed_over() {
    boot(&mut Run::new(report_keyboard_with(&KEYBOARD, [&[1], &[2, 1, 0], &[1, 2]])));
    boot(&mut Run::new(report_keyboard_with(&KEYBOARD, [&[], &[2, 1], &[1, 2]])));
}

#[test]
fn a_reference_to_a_report_the_map_has_not() {
    boot(&mut Run::new(report_keyboard_with(&KEYBOARD, [&[3, 1], &[2, 1], &[1, 2]])));
}

#[test]
fn a_reference_of_a_type_that_is_not_input() {
    boot(&mut Run::new(report_keyboard_with(&KEYBOARD, [&[1, 3], &[2, 1], &[1, 2]])));
    boot(&mut Run::new(report_keyboard_with(&KEYBOARD, [&[1, 0], &[2, 1], &[1, 2]])));
}

#[test]
fn volume_keys_alone_do_not_take_a_keyboard_out_of_boot_mode() {
    boot(&mut Run::new(report_keyboard_with(&KEYBOARD, [&[2, 1], &[2, 1], &[1, 2]])));
}
