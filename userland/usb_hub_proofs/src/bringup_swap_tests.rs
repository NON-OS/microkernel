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

//! A device swapped on a hub port between two looks: the port is still
//! connected, and only its connection change bit says the device is a new
//! one. The port stayed decided by the device before it, so the new one was
//! never brought up.

use crate::bringup_steps::{next, port, HIGH};
use nonos_usb_hub::Bringup;
use nonos_usb_hub::{HubPort, Step};

#[test]
fn a_device_swapped_between_two_looks_is_brought_up() {
    let mut b = Bringup::new();
    let (up, done) = (port(true, false, false), port(true, true, false));
    assert_eq!(next(&mut b, up, 0), (100, Step::Reset));
    assert_eq!(b.step(done, 110, true), Step::ResetOver(None));
    assert_eq!(next(&mut b, done, 111), (120, Step::Address(HIGH)));
    b.decide();
    assert_eq!(b.step(done, 5_000, true), Step::Wait, "the device there is decided");
    let swapped = HubPort { connect_changed: true, ..up };
    assert_eq!(b.step(swapped, 6_000, true), Step::Wait, "a new connection settles first");
    assert_eq!(next(&mut b, up, 6_001), (6_100, Step::Reset), "then is reset");
}
