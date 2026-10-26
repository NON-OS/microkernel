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

//! What can happen to the store's PROVE, and the one call a step makes.
//! Every timeout is an event, so the step decides what a deadline means.
//!
//! Pure; market_proofs holds it.

use super::proof_fail::Fail;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Event {
    /// A tick with nothing else to say.
    Tick,
    /// A deadline passed: the prover's start, or the job's quiet spell.
    Late,
    /// Op 8's answer, judged by proof_box.rs.
    Policy(Result<(), Fail>),
    /// MkProveJob's return.
    Job(i64),
    /// app.prove.job registered.
    Up,
    /// PROVE's reply: a job, or its status.
    Proved(Result<(), i32>),
    /// STATUS's reply: stage and total, or its status.
    Status(Result<(u8, u32), i32>),
    /// The STATUS call timed out: still working, and still outstanding.
    StatusLate,
    /// A GET's reply: its chunk's length, or its status.
    Got(Result<usize, i32>),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Call {
    None,
    Policy,
    AskJob,
    Lookup,
    Prove,
    Status,
    Get(u32),
}
