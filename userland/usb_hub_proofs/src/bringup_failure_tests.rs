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

//! A hub port reset that never ends fails at its 800 ms deadline, a device
//! that leaves mid way fails at once, and one left disabled says so.

use crate::bringup_steps::{next, port};
use nonos_usb_hub::Bringup;
use nonos_usb_hub::{HubError, HubPort, Step};

#[test]
fn a_reset_that_never_ends_fails_at_its_deadline_and_a_device_that_leaves_fails_at_once() {
    let mut b = Bringup::new();
    let (up, resetting) = (port(true, false, false), port(true, false, true));
    assert_eq!(next(&mut b, up, 0), (100, Step::Reset));
    assert_eq!(next(&mut b, resetting, 101), (900, Step::Failed(HubError::ResetTimeout)));
    assert!(!b.busy());
    assert_eq!(next(&mut b, up, 901), (1001, Step::Reset), "tried again");
    assert_eq!(b.step(port(false, false, false), 1002, true), Step::Failed(HubError::Gone));
    let disabled = port(true, false, false);
    let mut c = Bringup::new();
    assert_eq!(next(&mut c, disabled, 0), (100, Step::Reset));
    let over = HubPort { reset_changed: true, ..disabled };
    let not_enabled = Step::ResetOver(Some(HubError::NotEnabled));
    assert_eq!(next(&mut c, over, 101), (110, not_enabled), "read 10 ms after the reset");
}
