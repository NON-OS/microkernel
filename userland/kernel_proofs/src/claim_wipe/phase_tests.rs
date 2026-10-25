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

//! When a written reset's next step comes due (L78). Each window opens at its
//! end and not a nanosecond before, from any start; a clock read behind the
//! phase's own start, as one CPU's can be against another's, always waits,
//! however far behind.

use super::due_rule::{due, Due};
use super::phase_rule::{Phase, FLR_WAIT, MS, PM_WAIT, RETRY_WINDOW};

pub(super) fn draw(x: &mut u64) -> u64 {
    *x ^= *x << 13;
    *x ^= *x >> 7;
    *x ^= *x << 17;
    *x
}

/// Each written phase from `at`, its window, and what is due once it ends.
fn windows(at: u64) -> [(Phase, u64, Due); 3] {
    let (flr, d3, d0) = (Phase::Flr { at }, Phase::D3 { at }, Phase::D0 { at });
    [(flr, FLR_WAIT, Due::Check), (d3, PM_WAIT, Due::ToD0), (d0, PM_WAIT, Due::Check)]
}

#[test]
fn each_window_opens_at_its_end_not_a_nanosecond_before() {
    let (mut x, mut past_deadline) = (0x2545_F491_4F6C_DD1Du64, 0u32);
    for i in 0..20_000usize {
        let at = [0, 1, MS, u64::MAX / 4].get(i).copied().unwrap_or_else(|| draw(&mut x) >> 2);
        for (p, wait, open) in windows(at) {
            assert_eq!(due(p, at), Due::Wait, "{p:?} at once");
            assert_eq!(due(p, at + wait - 1), Due::Wait, "{p:?} a nanosecond early");
            assert_eq!(due(p, at + wait), open, "{p:?} at its end");
            assert_eq!(due(p, at + wait + 1), open, "{p:?} just after");
            let later = at + wait + draw(&mut x) % (10 * RETRY_WINDOW);
            assert_eq!(due(p, later), open, "{p:?} at {later}");
            past_deadline += u32::from(later >= at + wait + RETRY_WINDOW);
        }
    }
    assert!(past_deadline > 50_000, "{past_deadline} steps long after the window");
}

#[test]
fn a_clock_behind_the_phases_start_always_waits() {
    let (mut x, mut seen) = (0x9E37_79B9_7F4A_7C15u64, [0u32; 2]);
    for _ in 0..20_000 {
        let at = draw(&mut x) | 1;
        // Half a skew of a few windows, half as far behind as the clock can be.
        let far = draw(&mut x) & 1 == 0;
        let behind = 1 + draw(&mut x) % if far { at } else { at.min(4 * FLR_WAIT) };
        for (p, _, _) in windows(at) {
            assert_eq!(due(p, at - behind), Due::Wait, "{p:?} read {behind} ns behind");
        }
        seen[usize::from(behind > FLR_WAIT)] += 1;
    }
    assert!(seen[0] > 2_000 && seen[1] > 10_000, "near and far behind: {seen:?}");
    assert_eq!(due(Phase::Flr { at: u64::MAX }, 0), Due::Wait, "a wrapped difference is huge");
}
