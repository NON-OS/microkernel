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

//! When a failure counts as read: not on the answer that carries it, which
//! may go to a call that has given up, but on the owner's next frame.

use super::job_exit_tests::{failed, status, OWNER, T};
use crate::job::exit_rule::exits;
use nonos_proof_abi::wire::ReplyError;

/// The answer alone tells no one: the call it went to may have given up,
/// its reply then dropped. The owner's next frame says that call returned.
#[test]
fn a_failed_job_exits_once_its_asker_comes_back_after_the_answer() {
    let mut j = failed(-1102);
    assert!(!exits(Some(j.seen()), T + 1, T + 1), "it waits to be read");
    assert_eq!(status(&mut j, OWNER), Err(ReplyError::Status(-1102)));
    assert!(!exits(Some(j.seen()), T + 2, T + 2), "answered, perhaps to no one");
    j.heard(T + 3);
    assert_eq!(status(&mut j, OWNER), Err(ReplyError::Status(-1102)), "and told again");
    assert!(exits(Some(j.seen()), T + 4, T + 4), "read, it exits");
}

/// A frame before any failure was answered reads nothing.
#[test]
fn a_frame_before_the_answer_reads_no_failure() {
    let mut j = failed(-5);
    j.heard(T + 1);
    assert!(!exits(Some(j.seen()), T + 2, T + 2));
}
