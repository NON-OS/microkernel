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

//! The part B design reaches no bad world, H's death in a send included,
//! and each knob that undoes a piece of it is caught.

use super::search::search;
use super::world::{Knob, FOREIGN, STUCK};

#[test]
fn no_caller_sticks_and_no_send_carries_another_callers_sectors() {
    let f = search(Knob::Fixed);
    assert_eq!(f.bad, 0, "{f:?}");
    assert!(f.swept > 0 && f.ends > f.swept, "both H's death and its finish are reached: {f:?}");
}

#[test]
fn the_old_hold_leaves_pending_locked_by_a_dead_sender() {
    let f = search(Knob::SendUnderLock);
    assert_eq!(f.bad & STUCK, STUCK, "{f:?}");
}

#[test]
fn a_drop_that_drops_nothing_lets_a_reader_send_a_writers_sectors() {
    let f = search(Knob::NoDropHeld);
    assert_eq!(f.bad & FOREIGN, FOREIGN, "{f:?}");
}

#[test]
fn a_let_go_that_never_drops_lets_a_reader_send_a_writers_sectors() {
    let f = search(Knob::NoLetGo);
    assert_eq!(f.bad & FOREIGN, FOREIGN, "{f:?}");
}
