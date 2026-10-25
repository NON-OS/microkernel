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
//! One look by init's wait: done is done however late, out is a wait until
//! the bound, and at the bound init gives up and the drivers go on.

use crate::hardware::broker::acpi::first_pass_rule::{look, Look, FIRST_PASS_WAIT_MS};

#[test]
fn a_pass_in_ends_the_wait_however_late() {
    for waited in [0, 1, FIRST_PASS_WAIT_MS - 1, FIRST_PASS_WAIT_MS, u64::MAX] {
        assert_eq!(look(true, waited), Look::Done, "{waited} ms");
    }
}

#[test]
fn a_pass_out_is_waited_for_until_the_bound_and_never_past_it() {
    assert_eq!(look(false, 0), Look::Wait);
    assert_eq!(look(false, FIRST_PASS_WAIT_MS - 1), Look::Wait);
    assert_eq!(look(false, FIRST_PASS_WAIT_MS), Look::GaveUp);
    assert_eq!(look(false, u64::MAX), Look::GaveUp);
}
