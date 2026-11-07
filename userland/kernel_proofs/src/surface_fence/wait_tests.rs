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

//! Who waits on a surface's release fence: its owner, and for a guest's
//! pinned surface the guest's supervisor; nobody else.

use super::wait_rule::may_wait;

const OWNER: u32 = 40;
const GUEST: u32 = 41;
const SUPERVISOR: u32 = 30;

#[test]
fn the_owner_waits_on_its_own_surface() {
    assert!(may_wait(OWNER, OWNER, None));
}

#[test]
fn a_guest_surface_is_waited_on_by_the_guest_s_supervisor_alone() {
    assert!(may_wait(SUPERVISOR, GUEST, Some(SUPERVISOR)));
    for other in [1, 2, OWNER, 100, u32::MAX] {
        assert!(!may_wait(other, GUEST, Some(SUPERVISOR)), "{other}");
    }
}

#[test]
fn a_surface_that_is_no_guest_s_is_waited_on_by_its_owner_alone() {
    for other in [1, SUPERVISOR, GUEST, u32::MAX] {
        assert!(!may_wait(other, OWNER, None), "{other}");
    }
}

#[test]
fn no_process_is_group_zero() {
    assert!(!may_wait(0, 0, None));
    assert!(!may_wait(0, GUEST, Some(0)));
}
