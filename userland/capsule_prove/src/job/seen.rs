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

//! What the exit rule reads of a job (exit_rule.rs, state.rs).

use nonos_proof_abi::wire::Stage;

/// What the loop knows of its job.
#[derive(Clone, Copy, Debug)]
pub struct Seen {
    pub stage: Stage,
    /// GET has handed out the container's last byte.
    pub fetched: bool,
    /// The asker's call took an answer that carried the failure (state.rs).
    pub told: bool,
    /// When it failed, or the asker last sent a frame, whichever is later.
    pub asker_ms: u64,
    /// The worker thread is on its proofs (worker.rs).
    pub working: bool,
}
