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

//! Back to a display whose device came back: only to one that ranks above
//! the display driven, only when its driver is announced under a pid other
//! than the one that left, and the memory ends once a display at least as
//! good is driven.

use crate::display::return_rule::{back, remember, Left};
use crate::displays::DISPLAYS;

const GONE: u32 = 40;

fn left(rank: usize) -> Option<Left> {
    Some(Left { rank, pid: GONE })
}

#[test]
fn the_memory_survives_its_one_word() {
    assert_eq!(Left::word(None), 0);
    assert_eq!(Left::from_word(0), None);
    for rank in 0..DISPLAYS.len() {
        for pid in [0, 1, GONE, u32::MAX] {
            let k = Some(Left { rank, pid });
            assert_eq!(Left::from_word(Left::word(k)), k);
            assert_ne!(Left::word(k), 0, "a kept display is never nothing");
        }
    }
}

#[test]
fn the_better_ranked_display_left_is_kept() {
    let intel = Left { rank: 4, pid: GONE };
    let bootfb = Left { rank: 5, pid: 41 };
    assert_eq!(remember(None, intel), Some(intel));
    assert_eq!(remember(Some(intel), bootfb), Some(intel), "the fallback left too");
    let again = Left { rank: 4, pid: 50 };
    assert_eq!(remember(Some(intel), again), Some(again), "the same one left last");
}

#[test]
fn back_only_to_a_better_display_announced_by_another_pid() {
    let n = DISPLAYS.len();
    for rank in 0..n {
        for driving in 0..n {
            for announced in [None, Some(0), Some(GONE), Some(GONE + 1)] {
                let live = matches!(announced, Some(p) if p != 0 && p != GONE);
                let want = rank < driving && live;
                assert_eq!(
                    back(left(rank), driving, announced),
                    want,
                    "{rank} {driving} {announced:?}"
                );
            }
        }
        assert!(!back(None, n, Some(GONE + 1)), "nothing kept");
    }
}
