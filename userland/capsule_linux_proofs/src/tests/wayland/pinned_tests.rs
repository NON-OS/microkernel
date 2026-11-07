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

//! When the Wayland bridge shows a commit from its pinned buffer, and when
//! a buffer a layer drew is its client's again (present/pinned/rule.rs).

use crate::wayland_pinned::{
    given_back, gives_back, idle, paced, pinned, run, Facts, GIVE_BACK_MS, IDLE_MS, RUN_FRAMES,
    RUN_MS,
};

const ALL: Facts = Facts {
    on: true,
    layered: false,
    owes: false,
    same_shape: true,
    chrome_current: true,
    plain: true,
    pinnable: true,
    paced: false,
};

#[test]
fn a_window_as_it_is_with_a_buffer_a_pin_holds_is_shown_from_the_pin() {
    assert!(pinned(&ALL));
}

#[test]
fn any_fact_against_it_leaves_the_commit_to_the_copy() {
    let against = [
        Facts { on: false, ..ALL },
        Facts { layered: true, ..ALL },
        Facts { owes: true, ..ALL },
        Facts { same_shape: false, ..ALL },
        Facts { chrome_current: false, ..ALL },
        Facts { plain: false, ..ALL },
        Facts { pinnable: false, ..ALL },
        Facts { paced: true, ..ALL },
    ];
    for f in against {
        assert!(!pinned(&f), "{f:?}");
    }
}

#[test]
fn every_combination_but_one_is_copied() {
    for bits in 0u32..256 {
        let b = |i: u32| bits & (1 << i) != 0;
        let f = Facts {
            on: b(0),
            layered: !b(1),
            owes: !b(2),
            same_shape: b(3),
            chrome_current: b(4),
            plain: b(5),
            pinnable: b(6),
            paced: !b(7),
        };
        assert_eq!(pinned(&f), bits == 255, "{f:?}");
    }
}

#[test]
fn a_window_is_idle_once_it_showed_one_buffer_for_idle_ms() {
    assert!(!idle(1_000, 1_000));
    assert!(!idle(1_000, 1_000 + IDLE_MS - 1));
    assert!(idle(1_000, 1_000 + IDLE_MS));
    assert!(!idle(5_000, 1_000), "a clock read before the show is no wait");
    assert!(idle(0, u64::MAX));
}

#[test]
fn an_idle_window_gives_its_buffer_back_at_idle_with_no_spare_and_later_with_one() {
    assert!(gives_back(1_000, 1_000 + IDLE_MS, false));
    assert!(!gives_back(1_000, 1_000 + IDLE_MS, true), "a spare to draw in: kept");
    assert!(!gives_back(1_000, 1_000 + GIVE_BACK_MS - 1, true), "kept up to GIVE_BACK_MS");
    assert!(gives_back(1_000, 1_000 + GIVE_BACK_MS, true), "then given back all the same");
    assert!(gives_back(1_000, u64::MAX, true));
    assert!(!gives_back(1_000, 1_000 + IDLE_MS - 1, false), "not idle yet");
    for (since, now) in [(0, 0), (1_000, 1_249), (1_000, 1_250), (5_000, 1_000), (0, u64::MAX)] {
        assert_eq!(gives_back(since, now, false), idle(since, now));
        assert_eq!(gives_back(since, now, true), now.saturating_sub(since) >= GIVE_BACK_MS);
        assert!(!gives_back(since, now, true) || gives_back(since, now, false), "never sooner");
    }
}

#[test]
fn a_buffer_is_given_back_once_let_go_refused_or_waited_out() {
    assert!(!given_back(7, 7, 0), "still drawn");
    assert!(given_back(7, 8, 0), "its count moved");
    assert!(given_back(u32::MAX, 0, 0), "and wrapped");
    assert!(given_back(7, -22, 0), "the kernel has no such surface");
    assert!(!given_back(7, 7, GIVE_BACK_MS - 1));
    assert!(given_back(7, 7, GIVE_BACK_MS), "waited out");
}

#[test]
fn a_window_that_showed_no_commit_of_its_client_is_never_paced() {
    for now in [0, 1_000, u64::MAX] {
        for spare in [false, true] {
            for r in [0, 1, RUN_FRAMES, u32::MAX] {
                assert!(!paced(None, None, r, now, spare), "nothing committed to it yet");
            }
        }
    }
}

#[test]
fn a_run_counts_the_commits_in_a_row_each_sooner_than_run_ms_after_the_last() {
    assert_eq!(run(0, None, 1_000), 0, "no commit before it");
    assert_eq!(run(4, None, 1_000), 0);
    assert_eq!(run(0, Some(1_000), 1_016), 1);
    assert_eq!(run(2, Some(1_000), 1_000 + RUN_MS - 1), 3);
    assert_eq!(run(7, Some(1_000), 1_000 + RUN_MS), 0, "as long ago: a new start");
    assert_eq!(run(7, Some(1_000), u64::MAX), 0);
    assert_eq!(run(0, Some(5_000), 1_000), 1, "a clock read before the last commit is no wait");
    assert_eq!(run(u32::MAX, Some(1_000), 1_001), u32::MAX, "never wraps");
}

#[test]
fn a_client_with_no_spare_is_pinned_only_by_a_run_of_frames() {
    assert!(RUN_FRAMES >= 2, "foot's two commits for a key are no run");
    for last in [(Some(1_000), None), (None, Some(1_000)), (Some(1_000), Some(1_000))] {
        let (idle_back, own_last) = last;
        for r in 0..RUN_FRAMES {
            assert!(paced(idle_back, own_last, r, 1_016, false), "{last:?} {r}: copied, its box");
        }
        for r in [RUN_FRAMES, RUN_FRAMES + 1, u32::MAX] {
            assert!(!paced(idle_back, own_last, r, 1_016, false), "{last:?} {r}: pinned");
        }
    }
}

/// Each commit of a client with no spare at `times`, in order, on a window
/// that showed its first commit by the copy at 0: whether it was pinned.
fn pinned_at(times: &[u64]) -> alloc::vec::Vec<bool> {
    let (mut last, mut r, mut out) = (Some(0u64), 0u32, alloc::vec::Vec::new());
    for &t in times {
        r = run(r, last, t);
        let pin = !paced(None, last, r, t, false);
        out.push(pin);
        // Pinned, the window no longer shows its own buffers.
        (last, r) = if pin { (None, 0) } else { (Some(t), r) };
    }
    out
}

#[test]
fn keys_drawn_in_two_commits_each_are_never_pinned() {
    // foot on a guest: each key a commit and another 30 to 80 ms after it.
    let keys = [2_000, 2_032, 4_000, 4_078, 6_000, 6_031, 8_000, 8_040];
    assert!(pinned_at(&keys).iter().all(|p| !p), "{:?}", pinned_at(&keys));
    let typed = [2_000, 2_200, 2_400, 2_650, 2_900];
    assert!(pinned_at(&typed).iter().all(|p| !p), "a person's pace: each copied");
}

#[test]
fn a_run_of_frames_is_pinned_from_its_run_frames_th_quick_commit() {
    let frames: alloc::vec::Vec<u64> = (1..=10).map(|n| 5_000 + 16 * n).collect();
    let shown = pinned_at(&frames);
    // The first is 5 s after the last: a new start; then each 16 ms on.
    let first = shown.iter().position(|p| *p).expect("a run is pinned");
    assert_eq!(first, RUN_FRAMES as usize, "{shown:?}");
    assert!(shown[..first].iter().all(|p| !p));
}

#[test]
fn a_client_with_a_spare_is_paced_only_at_give_back_ms_whatever_its_run() {
    for r in [0, 1, RUN_FRAMES, u32::MAX] {
        for (i, o) in [(Some(1_000), None), (None, Some(1_000)), (Some(1_000), Some(1_000))] {
            assert!(!paced(i, o, r, 1_000 + IDLE_MS, true), "its pin kept: it draws in its spare");
            assert!(!paced(i, o, r, 1_000 + GIVE_BACK_MS - 1, true));
            assert!(paced(i, o, r, 1_000 + GIVE_BACK_MS, true), "given back by then: copied");
            assert!(!paced(i, o, r, 500, true), "a clock read before the last commit is no wait");
        }
    }
    for (last, now) in [(0, 0), (1_000, 1_999), (1_000, 2_000), (5_000, 1_000), (0, u64::MAX)] {
        assert_eq!(paced(None, Some(last), 0, now, true), gives_back(last, now, true));
    }
}

#[test]
fn a_commit_is_never_paced_sooner_with_a_spare_than_without() {
    let at = [None, Some(0), Some(1_000), Some(5_000)];
    for idle_back in at {
        for own_last in at {
            for now in [0, 1_016, 1_099, 1_100, 1_250, 1_999, 2_000, u64::MAX] {
                let r = run(0, own_last.or(idle_back), now);
                if paced(idle_back, own_last, r, now, true) {
                    assert!(
                        paced(idle_back, own_last, r, now, false),
                        "{idle_back:?} {own_last:?} {now}"
                    );
                }
            }
        }
    }
}

#[test]
fn keys_typed_at_a_person_s_pace_come_after_run_ms_and_before_idle_ms() {
    assert!(RUN_MS < IDLE_MS && IDLE_MS < GIVE_BACK_MS);
    for gap in [RUN_MS, 150, 200, IDLE_MS - 1] {
        assert_eq!(run(2, Some(1_000), 1_000 + gap), 0, "{gap} ms: no run");
        assert!(paced(Some(1_000), None, 0, 1_000 + gap, false), "{gap} ms: copied, its box");
        assert!(!idle(1_000, 1_000 + gap), "{gap} ms: sooner than the give-back");
    }
}
