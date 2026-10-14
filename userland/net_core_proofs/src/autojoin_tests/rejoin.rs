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
//! A link that was up and went down is joined again: within a bound, a
//! bounded number of times, and never after the passphrase was turned down.

use crate::autojoin_machine::{Due, Machine, Step, JOIN_WATCH_MS, LINK_EVERY_MS, REJOIN_ATTEMPTS};
use crate::autojoin_machine::{REJOIN_FIRST_MS, REJOIN_MAX_MS, WATCH_EVERY_MS};

/// Joined at boot, seen up, then seen down at `t`: the rejoin is under way.
pub(super) fn lost_at(t: i64) -> Machine {
    let mut m = Machine::new();
    m.after_attempt(0, Step::Answered { index: 1, joined: true });
    assert!(m.link_due(0) && !m.after_link(0, Some(true), false));
    assert!(m.after_link(t, Some(false), false) && m.rejoining());
    m
}

/// Run passes that do not hear the network until the machine is done or
/// `limit` passes ran; the passes and the time of the last.
pub(super) fn unheard(m: &mut Machine, mut t: i64, limit: u8) -> (u8, i64) {
    let mut passes = 0;
    while !m.done() && passes < limit {
        while m.due(t) != Due::Attempt {
            assert_eq!(m.due(t), Due::Nothing);
            t += 1_000;
        }
        m.after_attempt(t, Step::NoneInRange);
        passes += 1;
    }
    (passes, t)
}

#[test]
fn an_access_point_that_returns_is_joined_again_within_the_bound() {
    let mut m = lost_at(10_000);
    assert_eq!(m.due(10_000 + REJOIN_FIRST_MS - 1), Due::Nothing, "backs off first");
    let (passes, t) = unheard(&mut m, 10_000, 3);
    assert_eq!(passes, 3);
    assert!(t - 10_000 >= REJOIN_FIRST_MS * 7, "each wait twice the last");
    let t = (t..).find(|&t| m.due(t) == Due::Attempt).unwrap();
    m.after_attempt(t, Step::Started(1));
    assert_eq!(m.after_watch(t + WATCH_EVERY_MS, Some(true)), Some((1, true)));
    assert!(m.done() && !m.rejoining());
    // The bound: every pass's wait, then one join watched to its end.
    let waits: i64 = (0..REJOIN_ATTEMPTS).map(|n| (REJOIN_FIRST_MS << n).min(REJOIN_MAX_MS)).sum();
    assert!(t + WATCH_EVERY_MS - 10_000 <= waits + JOIN_WATCH_MS);
    // Joined again, the link is watched again, and a second loss rejoins.
    assert!(m.after_link(t + 2 * LINK_EVERY_MS, Some(false), false));
}

#[test]
fn an_access_point_that_never_returns_stops_the_attempts() {
    let mut m = lost_at(0);
    let (passes, t) = unheard(&mut m, 0, u8::MAX);
    assert_eq!(passes, REJOIN_ATTEMPTS);
    assert!(m.done() && !m.rejoining());
    assert_eq!(m.due(i64::MAX), Due::Nothing);
    // Still down: no new rejoin until a link is seen up (from Settings).
    assert!(m.link_due(t) && !m.after_link(t, Some(false), false));
    assert!(!m.after_link(t + LINK_EVERY_MS, Some(true), false));
    assert!(m.after_link(t + 2 * LINK_EVERY_MS, Some(false), false));
}
