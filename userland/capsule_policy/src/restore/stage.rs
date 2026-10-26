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

//! The restore as steps, one vfs ask each, so the server loop is never held.
//!
//! An ask vfs did not answer in time leaves the step where it was, to be
//! asked again on a later pass: a slow store makes the restore slow, never
//! finished early with defaults. No crate paths here, so the host proofs
//! mount this file as it is.

/// Where the restore stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    /// Waiting for vfs to load the store from disk.
    Settle,
    /// Reading setup's marker.
    Marker,
    /// Reading setup's answers.
    Answers,
    /// Reading what Settings kept after setup.
    Values,
    /// Done: what was kept is back, or nothing was kept.
    Finished,
}

/// What one ask got from vfs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Got {
    /// The store has loaded, or the file is there and whole.
    Yes,
    /// Not loaded yet, or the file is absent or refused.
    No,
    /// No answer in time.
    Late,
}

/// The stage after `stage` got `got`.
pub fn next(stage: Stage, got: Got) -> Stage {
    match (stage, got) {
        (_, Got::Late) => stage,
        (Stage::Settle, Got::Yes) => Stage::Marker,
        (Stage::Settle, Got::No) => Stage::Settle,
        (Stage::Marker, Got::Yes) => Stage::Answers,
        (Stage::Answers, Got::Yes) => Stage::Values,
        (Stage::Marker | Stage::Answers, Got::No) => Stage::Finished,
        (Stage::Values | Stage::Finished, _) => Stage::Finished,
    }
}
