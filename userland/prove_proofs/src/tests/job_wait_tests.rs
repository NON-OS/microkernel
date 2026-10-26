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

//! The receive waits no longer than the nearer exit deadline, and never 0,
//! which would wait for ever.

use crate::job::exit_rule::{exits, wait_ms, FAILED_QUIET_MS, QUIET_MS};
use crate::job::state::Job;
use nonos_proof_abi::wire::Stage;

const T: u64 = 1_000_000;

#[test]
fn any_job_or_none_exits_after_the_quiet_spell() {
    assert!(!exits(None, T + QUIET_MS, T));
    assert!(exits(None, T + QUIET_MS + 1, T));
    let running = Job::new(7, 40);
    assert!(exits(Some(running.seen()), T + QUIET_MS + 1, T));
}

#[test]
fn the_wait_reaches_the_nearer_deadline_and_is_never_0() {
    assert_eq!(wait_ms(None, T, T), QUIET_MS + 1);
    let mut j = Job::new(7, 40);
    (j.stage, j.asker_ms) = (Stage::Failed, T);
    assert_eq!(wait_ms(Some(j.seen()), T, T), FAILED_QUIET_MS + 1);
    // Waking at the deadline, exits says go; past it, the wait is still 1.
    assert!(exits(Some(j.seen()), T + FAILED_QUIET_MS + 1, T));
    assert_eq!(wait_ms(Some(j.seen()), T + 10 * QUIET_MS, T), 1);
}

/// Neither receive the job makes waits for ever: the drain between stages
/// looks once, and the serve loop waits what wait_ms gives.
#[test]
fn no_receive_of_the_job_waits_for_ever() {
    let drain = include_str!("../../../capsule_prove/src/job/drain.rs");
    let serve = include_str!("../../../capsule_prove/src/job/serve.rs");
    assert!(drain.contains("buf.len(), MK_IPC_NO_WAIT, &mut m)"));
    assert!(serve.contains("buf.len(), wait, &mut m)"));
    for text in [drain, serve] {
        assert!(!text.contains("buf.len(), 0, &mut m)"), "a receive waits for ever");
    }
}
