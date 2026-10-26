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

//! The worker's progress carried into the job: its stages while it works,
//! then Done with the container or Failed with the status, the failure's
//! 30 s starting now; a job not working is left alone.

use crate::job::exit_rule::{exits, wait_ms, POLL_MS, QUIET_MS};
use crate::job::poll_rule::carry;
use crate::job::state::Job;
use nonos_proof_abi::wire::Stage;

const T: u64 = 1_000_000;

fn working() -> Job {
    let mut j = Job::new(7, 40);
    (j.stage, j.working) = (Stage::Device, true);
    j
}

#[test]
fn the_job_takes_each_stage_the_worker_reports_then_its_result() {
    let mut j = working();
    carry(&mut j, Stage::Queued, None, T);
    assert_eq!(j.stage, Stage::Device, "a stage before the worker's first is not one");
    carry(&mut j, Stage::Isolation, None, T);
    assert_eq!((j.stage, j.working), (Stage::Isolation, true));
    carry(&mut j, Stage::Done, Some(Ok(vec![1, 2])), T + 5);
    assert_eq!(
        (j.stage, j.working, j.out.clone(), j.asker_ms),
        (Stage::Done, false, vec![1, 2], T + 5)
    );
    let mut f = working();
    carry(&mut f, Stage::Failed, Some(Err(-1102)), T + 9);
    assert_eq!((f.stage, f.status, f.working, f.asker_ms), (Stage::Failed, -1102, false, T + 9));
}

#[test]
fn a_job_not_working_is_left_alone() {
    let mut j = Job::new(7, 40);
    carry(&mut j, Stage::Isolation, Some(Ok(vec![1])), T);
    assert_eq!((j.stage, j.out.len()), (Stage::Queued, 0));
}

/// The quiet spell never ends a job mid-proof, and the loop looks at the
/// worker every POLL_MS.
#[test]
fn a_working_job_never_exits_and_is_looked_at_often() {
    let j = working();
    assert!(!exits(Some(j.seen()), T + 10 * QUIET_MS, T));
    assert_eq!(wait_ms(Some(j.seen()), T, T), POLL_MS);
    let mut done = working();
    carry(&mut done, Stage::Failed, Some(Err(-5)), T);
    assert_ne!(wait_ms(Some(done.seen()), T, T), POLL_MS);
}
