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
//! ends; a hub that goes away takes every device below it along, each
//! releasing what it held.

use super::devices::Devices;
use super::drop_port::drop_slot;
use super::forget_slot::forget_slot;
use crate::descriptors::HidKind;
use crate::endpoints::{ep, posted};
use crate::hub::{Hub, Place};
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
fn a_hub_unplugged_releases_what_every_device_below_it_held() {
    reset();
    let mut devs = Devices::new();
    let mut hub = Hub::new(4, Place::root(2, 3), 4);
    hub.child[0] = 5;
    devs.hubs.push(hub);
    devs.eps.push(ep(2, 5, 3, HidKind::Keyboard));
    devs.eps[0].dev.feed(&[0, 0, 0x05, 0, 0, 0, 0, 0]);
    let _ = posted();
    forget_slot(7, 4, &mut devs);
    assert_eq!(posted(), vec![(KEY_UP, u32::from(b'b'))]);
    assert!(devs.eps.is_empty() && devs.hubs.is_empty());
}
