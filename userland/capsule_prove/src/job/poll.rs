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

//! The serving thread looks at the worker (worker.rs): its stage, and once
//! it is done, its proofs made into the container (finish.rs), carried into
//! the job by poll_rule.rs.

use nonos_proof_abi::wire::PROVER;

use super::finish::Finish;
use super::poll_rule::carry;
use super::state::Job;
use super::worker::PROGRESS;

/// True when the worker's result came in now: the job has just ended.
pub fn poll(job: Option<&mut Job>, finish: &mut Option<Finish>, now: u64) -> bool {
    let Some(j) = job.filter(|j| j.working) else { return false };
    let result = PROGRESS.take().map(|made| {
        let f = finish.take();
        made.and_then(|m| f.as_ref().and_then(|f| f.container(&m)).ok_or(PROVER))
    });
    let ended = result.is_some();
    carry(j, PROGRESS.stage(), result, now);
    ended
}
