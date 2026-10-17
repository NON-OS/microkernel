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

//! What each stage of a claim allows (L65, L66). Pure, so the proofs run it
//! on the host beside a model of a release racing a grant.

use super::types::{ClaimError, Stage};

/// Whether a grant path may take the claim: only a made one not leaving.
pub(crate) const fn grantable(stage: Stage) -> bool {
    matches!(stage, Stage::Ready)
}

/// Whether a grant made against the claim may still be entered (L69): while
/// it is ready, or leaving with the paths its release waits out still inside.
pub(crate) const fn kept(stage: Stage) -> bool {
    matches!(stage, Stage::Ready | Stage::Leaving)
}

/// The stage a release moves the claim to, or why none may begin: a claim
/// still being made, its holder alive or not, is the claiming call's to settle.
pub(crate) const fn retired(stage: Stage) -> Result<Stage, ClaimError> {
    match stage {
        Stage::Making | Stage::Orphaned | Stage::Exiting => Err(ClaimError::NotClaimed),
        Stage::Ready | Stage::Leaving => Ok(Stage::Leaving),
    }
}

/// The stage an exit moves the claim to (L75), and whether the exit counts
/// among its releases: a claim still being made is orphaned for its call to
/// take back (L66); any other is the exit's to tear down.
pub(crate) const fn exited(stage: Stage) -> (Stage, bool) {
    match stage {
        Stage::Making | Stage::Orphaned => (Stage::Orphaned, false),
        Stage::Ready | Stage::Leaving | Stage::Exiting => (Stage::Exiting, true),
    }
}

/// Why a claim may not begin over an entry in `stage`: one being made or
/// standing is another's; one going is followed by its wipe, so ask again.
pub(crate) const fn over(stage: Stage) -> ClaimError {
    match stage {
        Stage::Making | Stage::Ready => ClaimError::AlreadyClaimed,
        Stage::Leaving | Stage::Orphaned | Stage::Exiting => ClaimError::Resetting,
    }
}

/// Whether the entry goes now: being released, no release left, no path
/// left inside it (L75).
pub(crate) const fn gone(stage: Stage, leavers: u32, inflight: u32) -> bool {
    matches!(stage, Stage::Leaving | Stage::Exiting) && leavers == 0 && inflight == 0
}
