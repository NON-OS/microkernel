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
//! W1 (c): only a lost link is rejoined. A Leave from Settings or the setup
//! wizard never is, before a rejoin or during one, and a join the driver ran
//! on that came back with a transient code is tried again within the bounds.

use super::rejoin::{lost_at, unheard};
use crate::autojoin_machine::{Due, Machine, Step, REJOIN_ATTEMPTS, REJOIN_FIRST_MS};

#[test]
fn a_leave_from_settings_or_the_wizard_is_never_rejoined() {
    let mut m = Machine::new();
    m.after_attempt(0, Step::Answered { index: 0, joined: true });
    assert!(!m.after_link(0, Some(true), false));
    assert!(!m.after_link(1, Some(false), true), "the person left: no rejoin");
    assert!(!m.after_link(2, Some(false), false), "and not on the next look either");
    assert_eq!(m.due(i64::MAX), Due::Nothing);
    assert!(!m.after_link(3, Some(true), false), "joined again from Settings");
    assert!(m.after_link(4, Some(false), false) && m.rejoining());
}

#[test]
fn a_leave_during_a_rejoin_ends_it() {
    let mut m = lost_at(0);
    let t = (0..).find(|&t| m.due(t) == Due::Attempt).unwrap();
    // The pass's look finds the person left: the glue ends the pass with Done.
    m.after_attempt(t, Step::Done);
    assert!(m.done() && !m.rejoining());
    // A driver that refused the rejoin itself (-13) ends it too.
    let mut m = lost_at(0);
    m.after_attempt(REJOIN_FIRST_MS, Step::Started(1));
    assert_eq!(m.join_ended(REJOIN_FIRST_MS + 500, -13), Some((1, false)));
    assert!(m.done() && !m.rejoining());
}

#[test]
fn a_join_run_on_then_a_transient_code_retries_within_the_bounds() {
    let mut m = lost_at(0);
    let (mut t, mut passes) = (0, 0u8);
    while m.rejoining() {
        t = (t..).find(|&t| m.due(t) == Due::Attempt).unwrap();
        m.after_attempt(t, Step::Started(1));
        passes += 1;
        assert_eq!(m.due(t + 1_000), Due::Watch, "the join runs on");
        assert_eq!(m.join_ended(t + 1_000, -2), Some((1, false)), "not heard: transient");
    }
    assert_eq!(passes, REJOIN_ATTEMPTS, "each transient code used one attempt");
    assert!(m.done() && m.due(i64::MAX) == Due::Nothing);
    // A join run on that came back joined, or refused, ends the rejoin at once.
    for (code, joined) in [(0, true), (-6, false), (-11, false)] {
        let mut m = lost_at(0);
        let (_, t) = unheard(&mut m, 0, 1);
        let t = (t..).find(|&t| m.due(t) == Due::Attempt).unwrap();
        m.after_attempt(t, Step::Started(1));
        assert_eq!(m.join_ended(t + 1, code), Some((1, joined)), "code {code}");
        assert!(m.done() && !m.rejoining());
    }
    let mut boot = Machine::new();
    boot.after_attempt(0, Step::Started(0));
    assert_eq!(boot.join_ended(1, -2), None, "at boot the watch runs on");
}
