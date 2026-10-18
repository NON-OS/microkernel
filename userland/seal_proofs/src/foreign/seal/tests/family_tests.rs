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

//! What a family says about its threads before any record is written.

use super::fixture::{family, SUP};
use crate::foreign::trap_table::Answer;

#[test]
fn a_thread_holds_its_call_until_the_call_closes() {
    let mut f = family(SUP);
    assert!(!f.holds(5));
    f.trapped(5, 0);
    assert!(f.holds(5) && !f.holds(6), "only the thread that trapped");
    f.answered(5, 0, Answer::Value(0));
    assert!(!f.holds(5), "answered");
    f.trapped(5, 1);
    f.leave(5);
    assert!(!f.holds(5), "closed as died");
}

#[test]
fn a_fresh_family_is_reserved_and_not_started() {
    assert!(!family(SUP).started, "slot 0 stays open until a guest starts");
}
