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

//! A failure is read when the answer carrying it went by MkIpcReplyTo and
//! the call took it (rc 0). One to a call that gave up reaches no
//! one, and the next STATUS carries it again; a later frame reads nothing.

use super::job_exit_tests::{failed, status, OWNER, T};
use crate::job::exit_rule::exits;
use crate::job::state::Job;
use nonos_proof_abi::wire::{ReplyError, Stage};

const ETIMEDOUT: i64 = -110;

#[test]
fn a_failed_job_exits_once_its_owner_took_the_answer() {
    let mut j = failed(-1102);
    assert!(!exits(Some(j.seen()), T + 1, T + 1), "it waits to be read");
    assert_eq!(status(&mut j, OWNER), Err(ReplyError::Status(-1102)));
    j.replied(0);
    assert!(exits(Some(j.seen()), T + 2, T + 2), "read, it exits");
}

#[test]
fn an_answer_no_call_took_tells_no_one_and_is_given_again() {
    let mut j = failed(-1102);
    for rc in [ETIMEDOUT, -2] {
        assert_eq!(status(&mut j, OWNER), Err(ReplyError::Status(-1102)));
        j.replied(rc);
        j.heard(T + 3);
        assert!(!exits(Some(j.seen()), T + 4, T + 4), "rc {rc}: reached no one");
    }
    assert_eq!(status(&mut j, OWNER), Err(ReplyError::Status(-1102)), "and told again");
    j.replied(0);
    assert!(exits(Some(j.seen()), T + 5, T + 5));
}

/// Only the answer that carried the failure, taken, reads it.
#[test]
fn only_the_answer_that_carried_the_failure_reads_it() {
    let mut j = Job { asker_ms: T, ..Job::new(7, OWNER) };
    assert!(status(&mut j, OWNER).is_ok(), "queued");
    j.replied(0);
    (j.stage, j.status) = (Stage::Failed, -5);
    assert!(!exits(Some(j.seen()), T + 2, T + 2), "no failure in that answer");
    assert_eq!(status(&mut j, OWNER + 1), Err(ReplyError::Status(-2)));
    j.replied(0);
    assert!(!exits(Some(j.seen()), T + 2, T + 2), "another pid's answer");
    assert_eq!(status(&mut j, OWNER), Err(ReplyError::Status(-5)));
    j.replied(ETIMEDOUT);
    j.replied(0);
    assert!(!exits(Some(j.seen()), T + 2, T + 2), "a later answer took");
}
