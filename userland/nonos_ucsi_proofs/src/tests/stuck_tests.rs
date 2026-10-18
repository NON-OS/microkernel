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

//! A PPM that stops answering: every wait bounded, the session reset and
//! walked again with growing pauses, and given up after three walks fail.

use super::session_tests::running;
use crate::model::Model;
use crate::poll::POLLS;
use crate::session::{Session, State, BACKOFF_MS};

#[test]
fn a_ppm_stuck_from_the_start_costs_one_bounded_wait() {
    let mut m = Model::new(2);
    m.stuck = true;
    assert!(Session::start(&mut m).is_err());
    assert_eq!(m.reads, POLLS, "the reset waited for, POLLS reads and no more");
}

#[test]
fn stuck_mid_session_then_back() {
    let (mut m, mut s) = running(2);
    m.plug(2, true, false);
    s.notified(&mut m, 100);
    m.stuck = true;
    s.tick(&mut m, 101);
    assert_eq!(s.state, State::Recovering { failures: 0, at: 101 });
    m.stuck = false;
    s.tick(&mut m, 101);
    assert_eq!(s.state, State::Running);
    assert!(s.ports[1].status.unwrap().connected, "the walk read what changed meanwhile");
    assert_eq!(s.take_changed(), 1 << 2);
}

#[test]
fn three_failed_walks_and_the_session_stops() {
    let (mut m, mut s) = running(1);
    m.plug(1, false, false);
    s.notified(&mut m, 0);
    m.stuck = true;
    s.tick(&mut m, 0);
    s.tick(&mut m, 0);
    assert_eq!(s.state, State::Recovering { failures: 1, at: BACKOFF_MS });
    let before = m.reads;
    s.tick(&mut m, BACKOFF_MS - 1);
    assert_eq!(m.reads, before, "nothing asked before the pause ends");
    s.tick(&mut m, BACKOFF_MS);
    assert_eq!(s.state, State::Recovering { failures: 2, at: BACKOFF_MS * 3 });
    s.tick(&mut m, BACKOFF_MS * 3);
    assert_eq!(s.state, State::Failed);
    s.tick(&mut m, u64::MAX);
    s.notified(&mut m, u64::MAX);
    assert_eq!(m.reads - before, 2 * POLLS, "a failed session reads nothing more");
}
