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

//! What the driver does not post: events it never asked for, and positions
//! while the host's integration is off.

use nonos_libc::posted;

use super::util::up;
use crate::serve::step;

// The host window resized: an event the filter keeps from ever flagging.
#[test]
fn events_outside_the_filter_never_flag_the_page() {
    let (mut d, host) = up();
    host.borrow_mut().host_resize();
    assert!(!host.borrow().pending_flag());
    assert!(!step(&mut d));
}

#[test]
fn with_integration_off_the_host_moves_the_ps2_pointer_and_nothing_is_posted() {
    let (mut d, host) = up();
    host.borrow_mut().host_move(10, 10);
    host.borrow_mut().integration = false;
    assert!(step(&mut d), "the pending event is still read");
    assert!(posted().is_empty());
    host.borrow_mut().host_move(20, 20);
    assert_eq!(host.borrow().ps2_moves, 1);
}
