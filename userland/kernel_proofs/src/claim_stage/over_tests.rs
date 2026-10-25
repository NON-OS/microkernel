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

//! Why a claim may not begin over an entry, for every stage (L78): one being
//! made or standing is another capsule's, so the claimant is told it is
//! taken; one on its way out is followed by its wipe, so the claimant is
//! told to ask again. A claim that can still be granted against is never
//! reported as passing, and a newly added stage stops the build here.

use super::stage_rule::{grantable, over};
use super::types::{ClaimError, Stage};

/// No catch-all arm: a stage added to the rule has to be placed here.
const fn want(stage: Stage) -> ClaimError {
    match stage {
        Stage::Making | Stage::Ready => ClaimError::AlreadyClaimed,
        Stage::Leaving | Stage::Orphaned | Stage::Exiting => ClaimError::Resetting,
    }
}

const STAGES: [Stage; 5] =
    [Stage::Making, Stage::Ready, Stage::Leaving, Stage::Orphaned, Stage::Exiting];

#[test]
fn an_entry_being_made_or_standing_is_taken_and_one_going_is_asked_again() {
    let mut told = [0u32; 2];
    for s in STAGES {
        let got = over(s);
        assert_eq!(got, want(s), "{s:?}");
        assert!(!grantable(s) || got == ClaimError::AlreadyClaimed, "{s:?} stands");
        told[(got == ClaimError::Resetting) as usize] += 1;
    }
    assert_eq!(told, [2, 3], "taken: Making, Ready; again: Leaving, Orphaned, Exiting");
    assert_eq!(over(Stage::Making), ClaimError::AlreadyClaimed);
    assert_eq!(over(Stage::Ready), ClaimError::AlreadyClaimed);
    assert_eq!(over(Stage::Leaving), ClaimError::Resetting);
    assert_eq!(over(Stage::Orphaned), ClaimError::Resetting);
    assert_eq!(over(Stage::Exiting), ClaimError::Resetting);
}
