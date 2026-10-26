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

//! What the worker's progress does to the job (poll.rs). Pure, so the host
//! runs it (prove_proofs::job_poll).
//!
//! While the worker is on its proofs, the job takes each stage it reports,
//! so STATUS says Device or Isolation at once. Its result ends the job:
//! Done with the container, or Failed with the status, and the failure's
//! 30 s for the asker to read it run from now (exit_rule.rs).

use alloc::vec::Vec;

use nonos_proof_abi::wire::Stage;

use super::state::Job;

/// `stage`: the one the worker published; `result`: its result, once it
/// has one, with the container already made of it.
pub fn carry(j: &mut Job, stage: Stage, result: Option<Result<Vec<u8>, i32>>, now: u64) {
    if !j.working {
        return;
    }
    let Some(result) = result else {
        if matches!(stage, Stage::Device | Stage::Isolation) {
            j.stage = stage;
        }
        return;
    };
    (j.working, j.asker_ms) = (false, now);
    match result {
        Ok(out) => (j.stage, j.out) = (Stage::Done, out),
        Err(status) => (j.stage, j.status) = (Stage::Failed, status),
    }
}
