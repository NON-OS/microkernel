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

//! The takeover's next step for no claim of bootfb and for each of its five
//! stages (L79), each answer stated from the design's reason for it, and
//! how `next` agrees with the stage rule: what it takes back is what a
//! release can begin on, and it is done exactly where none can.

use super::scanout_rule::{next, Next};
use super::stage_rule::{grantable, retired};
use super::types::Stage;

const STAGES: [Stage; 5] =
    [Stage::Making, Stage::Ready, Stage::Leaving, Stage::Orphaned, Stage::Exiting];

#[test]
fn the_takeover_is_done_where_no_bootfb_thread_can_write_the_scanout() {
    // No claim: nothing of bootfb's maps the window.
    assert_eq!(next(None), Next::Done);
    // Being made: nothing is granted against it yet, and it either meets the
    // covering claim's admit standing or is refused at its own once the stop
    // is latched.
    assert_eq!(next(Some(Stage::Making)), Next::Done);
    // Orphaned: its holder exited while it was made; it is never made ready.
    assert_eq!(next(Some(Stage::Orphaned)), Next::Done);
    // Exiting: every thread of its holder is off a CPU for good, and the
    // exit tears the claim down itself.
    assert_eq!(next(Some(Stage::Exiting)), Next::Done);
}

#[test]
fn a_standing_claim_is_taken_back_and_one_being_released_is_waited_out() {
    // Ready: its holder may be storing through its pages at this moment.
    assert_eq!(next(Some(Stage::Ready)), Next::TakeBack);
    // Leaving: a release is already tearing its pages down; the takeover
    // waits for the entry to go rather than finish early.
    assert_eq!(next(Some(Stage::Leaving)), Next::Wait);
}

#[test]
fn next_takes_back_only_what_a_release_can_begin_on() {
    for s in STAGES {
        let n = next(Some(s));
        // A take-back starts with retire: it must be one retire accepts.
        assert_eq!(n == Next::TakeBack, grantable(s) && retired(s).is_ok(), "{s:?}");
        // Done exactly where no release can begin: nothing left to wait on.
        assert_eq!(n == Next::Done, retired(s).is_err(), "{s:?}");
        // Waited on where a release is already under way.
        assert_eq!(n == Next::Wait, retired(s).is_ok() && !grantable(s), "{s:?}");
    }
}
