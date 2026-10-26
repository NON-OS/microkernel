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

//! A failed job exits once its asker has read the code (job_told_tests.rs),
//! or after 30 s of silence; a Done job exits once fetched, as before.

use crate::job::answer::answer;
use crate::job::exit_rule::{exits, FAILED_QUIET_MS};
use crate::job::state::Job;
use nonos_proof_abi::wire::{reply_body, request, Header, ReplyError, Stage, PROVE_MAGIC, STATUS};

pub(super) const OWNER: u32 = 40;
pub(super) const T: u64 = 1_000_000;

pub(super) fn failed(code: i32) -> Job {
    let mut j = Job::new(7, OWNER);
    (j.stage, j.status, j.asker_ms) = (Stage::Failed, code, T);
    j
}

/// `sender`'s STATUS for job 7, answered by the shipped `answer`.
pub(super) fn status(j: &mut Job, sender: u32) -> Result<Vec<u8>, ReplyError> {
    let rx = request(PROVE_MAGIC, STATUS, 1, &7u32.to_le_bytes());
    let (h, body) = Header::parse(&rx, PROVE_MAGIC).expect("a STATUS frame");
    let tx = answer(Some(j), sender, &h, body);
    reply_body(&tx, PROVE_MAGIC, STATUS, 1).map(<[u8]>::to_vec)
}

#[test]
fn another_pids_status_neither_reads_the_failure_nor_frees_the_prover() {
    let mut j = failed(-5);
    assert_eq!(status(&mut j, OWNER + 1), Err(ReplyError::Status(-2)));
    assert!(!exits(Some(j.seen()), T + 1, T + 1));
}

#[test]
fn a_failed_job_left_unread_exits_after_30_s() {
    let j = failed(-5);
    assert!(!exits(Some(j.seen()), T + FAILED_QUIET_MS, T + 1));
    assert!(exits(Some(j.seen()), T + FAILED_QUIET_MS + 1, T + 1));
    // A frame from the asker starts the 30 s again.
    let later = Job { asker_ms: T + 20_000, ..failed(-5) };
    assert!(!exits(Some(later.seen()), T + FAILED_QUIET_MS + 1, T + 1));
}

#[test]
fn a_done_job_exits_once_fetched_and_not_at_30_s() {
    let mut j = Job::new(7, OWNER);
    (j.stage, j.asker_ms) = (Stage::Done, T);
    assert!(!exits(Some(j.seen()), T + FAILED_QUIET_MS + 1, T + FAILED_QUIET_MS));
    j.fetched = true;
    assert!(exits(Some(j.seen()), T + 1, T + 1));
}
