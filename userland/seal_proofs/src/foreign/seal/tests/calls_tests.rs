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

//! plan/10's row table: each answer closes its call once, on the side the
//! table names, under the K code the table names.

use super::fixture::{family, SUP};
use crate::arch::context::SavedUser;
use crate::foreign::seal::hist::slot;
use crate::foreign::trap_table::Answer;

const EPERM: u64 = -1i64 as u64;
const ENOSYS: u64 = -38i64 as u64;

#[test]
fn only_eperm_and_enosys_are_refusals() {
    let mut f = family(SUP);
    for v in [EPERM, ENOSYS, -9i64 as u64, -13i64 as u64, 0, 7] {
        f.trapped(7, 1);
        f.answered(7, 1, Answer::Value(v));
    }
    assert_eq!((f.hist[slot(1, true)], f.hist[slot(1, false)], f.rows), (2, 4, 6));
}

#[test]
fn an_answer_that_is_no_value_is_a_success() {
    let mut f = family(SUP);
    let answers = [Answer::Execed, Answer::Deliver(SavedUser), Answer::Sigreturn(SavedUser)];
    for (pid, a) in (1..).zip(answers) {
        f.trapped(pid, 59);
        f.answered(pid, 59, a);
    }
    assert_eq!((f.hist[slot(59, false)], f.hist[slot(59, true)]), (3, 0));
}

#[test]
fn an_answer_for_another_call_closes_nothing() {
    let mut f = family(SUP);
    f.trapped(7, 1);
    f.answered(7, 2, Answer::Value(0));
    f.answered(8, 1, Answer::Value(0));
    assert_eq!((f.rows, f.open.len()), (0, 1));
}

#[test]
fn a_failed_park_is_refused_and_a_death_is_not() {
    let mut f = family(SUP);
    f.trapped(7, 101);
    f.park_failed(7, 101);
    f.trapped(8, 101);
    f.trapped(8, 1); // its first call never got an answer it took
    f.leave(8);
    f.trapped(9, 600);
    f.leave_all();
    assert_eq!((f.hist[slot(101, true)], f.hist[slot(101, false)]), (1, 1));
    assert_eq!((f.hist[slot(1, false)], f.hist[slot(512, false)], f.rows), (1, 1, 4));
    assert!(f.open.is_empty());
}
