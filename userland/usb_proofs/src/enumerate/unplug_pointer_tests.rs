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

//! A pointer that goes away lets go of every button it held, so a drag
//! ends; a device below a hub, on the virtual port driver.usb_hub0 gives
//! it, lets go of what it held when that port empties, as the hub's going
//! empties the ports of every device below it; and a gamepad lets its
//! buttons up.

use super::drop_port::{drop_port, drop_slot};
use crate::descriptors::HidKind;
use crate::endpoints::{ep, posted};
use nonos_libc::INPUT_KIND_PAD_BUTTON_UP as PAD_UP;
use nonos_libc::{reset, INPUT_KIND_BUTTON_UP as BUTTON_UP, INPUT_KIND_KEY_UP as KEY_UP};

#[test]
fn a_mouse_or_tablet_with_a_button_held_lets_go_of_it() {
    reset();
    let mut eps = vec![ep(1, 1, 3, HidKind::Mouse), ep(1, 1, 5, HidKind::Report)];
    eps[0].dev.feed(&[0x01, 0, 0]);
    eps[1].dev.feed(&[0x04, 0, 0x40, 0, 0x40, 0]);
    let _ = posted();
    drop_slot(&mut eps, 1);
    assert_eq!(posted(), vec![(BUTTON_UP, 1), (BUTTON_UP, 3)]);
    assert!(eps.is_empty());
}

#[test]
fn a_device_below_a_hub_lets_go_when_its_virtual_port_empties() {
    reset();
    // Virtual port 40, the keyboard's, and 41 beside it.
    let mut eps = vec![ep(40, 5, 3, HidKind::Keyboard), ep(41, 6, 3, HidKind::Mouse)];
    eps[0].dev.feed(&[0, 0, 0x05, 0, 0, 0, 0, 0]);
    let _ = posted();
    assert_eq!(drop_port(&mut eps, 40), vec![5]);
    assert_eq!(posted(), vec![(KEY_UP, u32::from(b'b'))]);
    assert_eq!(eps.len(), 1);
}

#[test]
fn a_gamepad_unplugged_lets_its_buttons_up() {
    reset();
    let mut eps = vec![ep(2, 4, 3, HidKind::Gamepad)];
    eps[0].dev.feed(&[1, 0x01, 0x08, 0, 0, 0, 0, 0]);
    let _ = posted();
    drop_slot(&mut eps, 4);
    assert_eq!(posted(), vec![(PAD_UP, 1), (PAD_UP, 12)]);
}
