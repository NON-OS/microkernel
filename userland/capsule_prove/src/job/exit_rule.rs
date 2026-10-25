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

//! When the headless job exits, and how long its receive may wait (plan/14;
//! the I-B9a ruling of 2026-10-25). Pure, so the host runs it
//! (prove_proofs::job_exit).
//!
//! - Done: once its container's last byte is fetched.
//! - Failed: once its asker's STATUS has carried the failure, or after
//!   FAILED_QUIET_MS with nothing from the asker, so a failure never holds
//!   the prover from the next ask.
//! - Any job, or none: after QUIET_MS with no request at all.

use nonos_proof_abi::wire::Stage;

/// No PROVE within this long of starting, or no request this long after
/// the last: the job was abandoned.
pub const QUIET_MS: u64 = 10 * 60 * 1000;
/// A failed job waits this long after the failure, or after its asker's
/// last frame, for the asker to read it.
pub const FAILED_QUIET_MS: u64 = 30 * 1000;

/// What the loop knows of its job.
#[derive(Clone, Copy, Debug)]
pub struct Seen {
    pub stage: Stage,
    /// GET has handed out the container's last byte.
    pub fetched: bool,
    /// A STATUS from the asker has been answered with the failure.
    pub told: bool,
    /// When it failed, or the asker last sent a frame, whichever is later.
    pub asker_ms: u64,
}

/// Whether the job exits now; `last` is when any request last came.
pub fn exits(job: Option<Seen>, now: u64, last: u64) -> bool {
    if now.saturating_sub(last) > QUIET_MS {
        return true;
    }
    match job {
        Some(j) if j.stage == Stage::Done => j.fetched,
        Some(j) if j.stage == Stage::Failed => {
            j.told || now.saturating_sub(j.asker_ms) > FAILED_QUIET_MS
        }
        _ => false,
    }
}

/// How long the receive may wait before `exits` should be asked again: up
/// to the nearer deadline, at least 1 ms, since 0 would wait for ever.
pub fn wait_ms(job: Option<Seen>, now: u64, last: u64) -> u64 {
    let quiet = (last.saturating_add(QUIET_MS + 1)).saturating_sub(now);
    let failed = match job {
        Some(j) if j.stage == Stage::Failed => {
            (j.asker_ms.saturating_add(FAILED_QUIET_MS + 1)).saturating_sub(now)
        }
        _ => u64::MAX,
    };
    quiet.min(failed).max(1)
}
