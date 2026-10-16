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

/// The stage a release moves the claim to, or why none may begin: a claim
/// still being made, its holder alive or not, is the claiming call's to settle.
pub(crate) const fn retired(stage: Stage) -> Result<Stage, ClaimError> {
    match stage {
        Stage::Making | Stage::Orphaned => Err(ClaimError::NotClaimed),
        Stage::Ready | Stage::Leaving => Ok(Stage::Leaving),
    }
}
