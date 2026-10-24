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

//! The poll's use of the return rule: whatever was kept, driven and
//! announced, a poll never goes back to a worse display and never keeps one
//! it should not go back to.

use crate::display::return_rule::{back, worth, Left};
use crate::displays::DISPLAYS;

const GONE: u32 = 40;

/// The poll's own composition (setup/move_back.rs): what is not worth going
/// back to is forgotten, and a move back forgets the one gone back to.
#[test]
fn after_any_poll_nothing_kept_ranks_at_or_below_the_display_driven() {
    let n = DISPLAYS.len();
    for rank in 0..n {
        for driving in 0..n {
            for announced in [None, Some(GONE), Some(GONE + 1)] {
                let kept = worth(Some(Left { rank, pid: GONE }), driving);
                let (kept, now) = match kept {
                    Some(k) if back(kept, driving, announced) => (None, k.rank),
                    _ => (kept, driving),
                };
                assert!(now <= driving, "never back to a worse display");
                assert!(kept.is_none_or(|k| k.rank < now), "{rank} {driving} {announced:?}");
            }
        }
    }
}
