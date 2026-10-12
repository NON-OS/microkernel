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

//! A PPM that notifies without end: notifications read at a bounded rate,
//! then a quiet spell, then every connector read once.

use super::session_tests::running;
use crate::storm::{LIMIT, QUIET_MS, WINDOW_MS};

#[test]
fn a_flood_is_read_at_a_bounded_rate() {
    let (mut m, mut s) = running(3);
    let before = m.reads;
    for i in 0..10_000u64 {
        m.plug((i % 3) as u8 + 1, i % 2 == 0, false);
        s.notified(&mut m, i / 10);
    }
    let read = m.reads - before;
    assert!(read <= LIMIT * 2, "{read} reads for 10 000 notifications in one second");
    assert_eq!(s.storm.storms, 1);
}

#[test]
fn after_the_quiet_every_connector_is_read_once() {
    let (mut m, mut s) = running(3);
    for _ in 0..=LIMIT {
        s.notified(&mut m, 0);
    }
    assert!(s.storm.quiet(1));
    m.plug(3, true, false);
    s.notified(&mut m, 1);
    s.tick(&mut m, 1);
    assert!(!s.ports[2].status.unwrap().connected, "nothing read while quiet");
    s.tick(&mut m, QUIET_MS);
    assert!(s.ports[2].status.unwrap().connected);
    let reads = m.reads;
    s.tick(&mut m, QUIET_MS + 1);
    assert_eq!(m.reads, reads, "the rescan happens once");
    assert!(s.storm.admit(QUIET_MS + WINDOW_MS), "notifications read again");
}

#[test]
fn a_connector_the_ppm_has_no_port_for_is_dropped() {
    let (mut m, mut s) = running(1);
    let sent = m.commands.len();
    s.pending = 1 << 9;
    s.tick(&mut m, 0);
    assert_eq!(s.pending, 0);
    assert_eq!(m.commands.len(), sent, "no command for connector 9");
}
