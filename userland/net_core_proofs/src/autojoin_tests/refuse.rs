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
//! A rejoin never repeats a join that may have failed on the passphrase, and
//! nothing but a link seen up and then down starts one.

use super::rejoin::{lost_at, unheard};
use crate::autojoin_machine::{Due, Machine, Step, JOIN_WATCH_MS, REJOIN_FIRST_MS};

#[test]
fn a_wrong_passphrase_is_never_retried() {
    let mut m = lost_at(0);
    let (_, t) = unheard(&mut m, 0, 2);
    let t = (t..).find(|&t| m.due(t) == Due::Attempt).unwrap();
    m.after_attempt(t, Step::Refused(1));
    assert!(m.done() && !m.rejoining(), "ends at the first refusal");
    assert_eq!(m.due(i64::MAX), Due::Nothing);
}

// The driver gives no reason for a join it ran on that did not come up; it
// may have been the passphrase, so that join is not handed over again.
#[test]
fn a_join_run_on_that_never_came_up_is_not_repeated() {
    let mut m = lost_at(0);
    m.after_attempt(REJOIN_FIRST_MS, Step::Started(1));
    let end = REJOIN_FIRST_MS + JOIN_WATCH_MS;
    assert_eq!(m.after_watch(end - 1, Some(false)), None);
    assert_eq!(m.after_watch(end, None), Some((1, false)));
    assert!(m.done() && !m.rejoining());
    assert_eq!(m.due(i64::MAX), Due::Nothing);
}

#[test]
fn only_a_link_seen_up_then_down_starts_a_rejoin() {
    let mut m = Machine::new();
    assert!(!m.link_due(0), "not while boot's autojoin runs");
    assert!(!m.after_link(0, Some(false)), "never up");
    m.after_attempt(0, Step::Done);
    assert!(m.link_due(0) && !m.after_link(0, Some(false)));
    // A driver that does not answer in time says nothing about the link.
    assert!(!m.after_link(1, Some(true)) && !m.after_link(2, None));
    assert!(!m.rejoining());
    // A clock that runs backwards neither overflows nor rejoins early.
    assert!(m.after_link(i64::MIN, Some(false)));
    assert_eq!(m.due(i64::MIN), Due::Nothing);
    assert_eq!(lost_at(i64::MAX).due(i64::MAX), Due::Attempt, "saturates at the end of time");
}
