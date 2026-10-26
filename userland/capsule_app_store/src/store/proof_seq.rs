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

//! Where the store's PROVE stands (I-B9a, section 3): Policy -> Boxed ->
//! Asked -> Up -> Proving -> Fetching -> Held, or Failed. What can happen,
//! and the calls, are proof_event.rs's; the step is proof_step.rs.
//!
//! Pure; market_proofs holds it.

use super::proof_fail::Fail;

/// wire/status.rs's stages, held to it by proof_call.rs.
pub const QUEUED: u8 = 0;
pub const READING: u8 = 1;
pub const DEVICE: u8 = 2;
pub const ISOLATION: u8 = 3;
pub const DONE: u8 = 4;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    /// Op 8 is to be asked for the listing's box.
    Policy,
    /// The box names this listing; MkProveJob is to be asked.
    Boxed,
    /// Init is starting the prover; app.prove.job is looked up a tick at a time.
    Asked,
    /// app.prove.job is up: PROVE is sent.
    Up,
    /// The job's last stage told, and whether a STATUS is outstanding: at
    /// most one is (ruling 3), and one that timed out is given up.
    Proving {
        stage: u8,
        out: bool,
    },
    /// GETs from `at`, to exactly `total`.
    Fetching {
        at: u32,
        total: u32,
    },
    Held,
    /// Why, and whether the run was read first, so is used up.
    Failed(Fail, bool),
}

/// Nothing more will happen to it.
pub fn ended(phase: Phase) -> bool {
    matches!(phase, Phase::Held | Phase::Failed(..))
}
