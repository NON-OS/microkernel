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

//! A debounced hub port waits its turn while another of the host's ports is
//! reset or recovering, so one device at a time answers at address 0; its
//! debounce is not started over, and it is reset once its turn comes.

use nonos_usb_hub::{Bringup, Step};

use crate::bringup_steps::{next, port};

#[test]
fn a_debounced_port_is_reset_only_in_its_turn() {
    let (mut a, mut b) = (Bringup::new(), Bringup::new());
    let up = port(true, false, false);
    assert_eq!(next(&mut a, up, 0), (100, Step::Reset));
    assert!(a.resetting());
    for t in 0..500 {
        assert_eq!(b.step(up, t, false), Step::Wait, "held while the other resets, at {t}");
    }
    assert!(!b.resetting() && b.busy(), "debounced, waiting");
    assert_eq!(b.step(up, 500, true), Step::Reset, "its turn: reset at once");
}
