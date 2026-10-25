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

use super::stage_rule::{exited, gone, grantable, kept, retired};
use super::types::{ClaimError, Stage};

#[test]
fn only_a_made_claim_not_leaving_is_granted_against() {
    assert!(grantable(Stage::Ready));
    assert!(!grantable(Stage::Making) && !grantable(Stage::Leaving));
    assert!(!grantable(Stage::Orphaned));
    assert_eq!(retired(Stage::Making), Err(ClaimError::NotClaimed), "the claiming call's");
    assert_eq!(retired(Stage::Orphaned), Err(ClaimError::NotClaimed), "an exit's leftover");
    assert_eq!(retired(Stage::Ready), Ok(Stage::Leaving));
    assert_eq!(retired(Stage::Leaving), Ok(Stage::Leaving), "a sibling's release waits too");
}

#[test]
fn a_grant_is_entered_only_while_its_claim_stands() {
    assert!(kept(Stage::Ready) && kept(Stage::Leaving), "a release waits these out");
    assert!(!kept(Stage::Making), "nothing is granted before the claim is made");
    assert!(!kept(Stage::Orphaned), "an exit's leftover takes no grant");
}

#[test]
fn an_exit_takes_every_made_claim_and_leaves_one_being_made_to_its_call() {
    assert!(!grantable(Stage::Exiting) && !kept(Stage::Exiting), "nothing granted or entered");
    assert_eq!(retired(Stage::Exiting), Err(ClaimError::NotClaimed), "the exit's to tear down");
    for s in [Stage::Ready, Stage::Leaving, Stage::Exiting] {
        assert_eq!(exited(s), (Stage::Exiting, true), "{s:?}");
    }
    assert_eq!(exited(Stage::Making), (Stage::Orphaned, false));
    assert_eq!(exited(Stage::Orphaned), (Stage::Orphaned, false));
}

#[test]
fn an_entry_goes_only_with_the_last_release_and_the_last_path_inside() {
    for s in [Stage::Leaving, Stage::Exiting] {
        assert!(gone(s, 0, 0), "{s:?}");
        assert!(!gone(s, 1, 0) && !gone(s, 0, 1), "{s:?}");
    }
    for s in [Stage::Making, Stage::Ready, Stage::Orphaned] {
        assert!(!gone(s, 0, 0), "{s:?} is its call's or its holder's");
    }
}
