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

//! A session brought up, then kept current as partners come and go.

use crate::model::Model;
use crate::session::{Session, State};

pub fn running(ports: usize) -> (Model, Session) {
    let mut m = Model::new(ports);
    m.plug(1, true, false);
    m.changes.clear();
    let s = Session::start(&mut m).expect("starts");
    (m, s)
}

#[test]
fn start_walks_every_connector() {
    let (m, s) = running(2);
    // Port 1 is a Power Delivery charger, so its offers are asked (0x10).
    let walk = [1, 5, 4, 6, 4, 7, 4, 0x12, 4, 0x10, 4, 7, 4, 0x12, 4];
    assert_eq!(m.commands, walk, "each acknowledged");
    assert_eq!(s.ports.len(), 2);
    let p1 = s.ports[0].status.unwrap();
    assert!(p1.connected && p1.charger());
    assert!(!s.ports[1].status.unwrap().connected);
    assert!(s.ports[0].capability.unwrap().drp);
    assert_eq!((s.pending, s.state), (0, State::Running));
}

#[test]
fn a_change_is_read_and_acknowledged() {
    let (mut m, mut s) = running(2);
    m.plug(2, true, true);
    s.notified(&mut m, 10);
    assert_eq!(s.pending, 1 << 2);
    s.tick(&mut m, 11);
    let p2 = s.ports[1].status.unwrap();
    assert!(p2.connected && p2.provider && !p2.charger());
    assert!(m.changes.is_empty(), "the change acknowledged with the read");
    assert_eq!((s.take_changed(), s.take_changed()), (1 << 2, 0));
}

#[test]
fn a_slow_ppm_is_waited_for() {
    let mut m = Model::new(1);
    m.delay = 20;
    let s = Session::start(&mut m).expect("starts");
    assert_eq!(s.ports.len(), 1);
    assert!(m.pauses >= 20 * 5, "paused between reads, not spun");
}

#[test]
fn a_refused_status_leaves_the_port_unknown_not_the_session() {
    let (mut m, mut s) = running(1);
    m.refuse_status = true;
    m.plug(1, false, false);
    s.notified(&mut m, 5);
    s.tick(&mut m, 6);
    assert_eq!((s.ports[0].status, s.state), (None, State::Running));
}
