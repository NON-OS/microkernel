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

//! The pairing method from both IO capabilities (Vol 3, Part H, 2.3.5.1),
//! as this host meets the devices Decision 21 serves: a keyboard and a
//! mouse. The host is KeyboardDisplay.

use nonos_bt_smp::method::{select, IoCap::*, Method::*};

#[test]
fn a_keyboard_types_the_passkey_the_host_shows() {
    assert_eq!(select(KeyboardDisplay, true, KeyboardOnly, true), PasskeyInitiatorDisplays);
    assert_eq!(select(KeyboardDisplay, true, KeyboardOnly, false), PasskeyInitiatorDisplays);
}

#[test]
fn a_mouse_and_no_mitm_are_just_works() {
    assert_eq!(select(KeyboardDisplay, true, NoInputNoOutput, false), JustWorks);
    assert_eq!(select(KeyboardDisplay, false, KeyboardOnly, false), JustWorks);
}

#[test]
fn the_rest_of_the_table() {
    assert_eq!(select(KeyboardDisplay, true, DisplayYesNo, true), NumericComparison);
    assert_eq!(select(DisplayYesNo, true, KeyboardDisplay, true), NumericComparison);
    assert_eq!(select(KeyboardDisplay, true, DisplayOnly, true), PasskeyResponderDisplays);
    assert_eq!(select(KeyboardOnly, true, DisplayYesNo, true), PasskeyResponderDisplays);
    assert_eq!(select(KeyboardOnly, true, KeyboardOnly, true), PasskeyBothInput);
    assert_eq!(select(DisplayOnly, true, DisplayYesNo, true), JustWorks);
    assert_eq!(select(DisplayOnly, true, KeyboardDisplay, true), PasskeyInitiatorDisplays);
}
