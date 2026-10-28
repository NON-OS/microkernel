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

//! With no circuit whose exit takes its port, a stream waits while one is
//! drawn for it, room is made if three are held, and the wait is bounded.

use crate::cell::Cell;
use crate::manager::Bootstrap;
use crate::stream::WAIT_SECONDS;

use super::exit_circuit::LINES;
use super::exit_far::BEGIN;
use super::exit_net::net;

#[test]
fn the_stream_waits_and_the_next_circuit_is_drawn_for_its_port() {
    let mut n = net(&LINES);
    n.circuit(0);
    let id = n.open(18081, false).expect("it waits rather than failing");
    assert_eq!(n.stream(id), (0, "opening".into()));
    assert!(n.look().is_empty(), "no BEGIN to an exit that refuses the port");
    n.tick(1);
    let drawn = n.state.circuits.last().unwrap();
    assert!(n.state.exits.takes(&drawn.path[2], 18081) && !drawn.warm, "its own, not the warm");
    let up = n.bring_up();
    n.tick(1);
    assert_eq!(n.stream(id), (up, "opening".into()));
    let begins: std::vec::Vec<_> = n.look().into_iter().filter(|s| s.1 == BEGIN).collect();
    assert_eq!((begins.len(), begins[0].0, begins[0].2), (1, up, id));
}

#[test]
fn with_three_held_an_idle_one_makes_room_and_a_busy_one_never_does() {
    let mut n = net(&LINES);
    let held = [n.circuit(0), n.circuit(3), n.circuit(4)];
    let web = n.open(443, false).unwrap();
    let busy = n.stream(web).0;
    let id = n.open(18081, false).unwrap();
    n.tick(1);
    let gone: std::vec::Vec<u32> =
        held.into_iter().filter(|c| !n.state.circuits.iter().any(|k| k.id == *c)).collect();
    assert_eq!(gone.len(), 1, "one idle circuit went");
    assert_ne!(gone[0], busy, "never the one carrying a stream");
    let sent = &n.state.link.as_ref().unwrap().sent;
    let destroy = |c: Cell| c.command == 4 && c.circuit == gone[0];
    assert!(sent.iter().filter_map(|b| Cell::decode(b)).any(destroy), "with a DESTROY");
    assert!(n.state.exits.takes(&n.state.circuits.last().unwrap().path[2], 18081));
    assert_eq!(n.stream(id).0, 0, "still waiting for it to come up");
}

#[test]
fn a_stream_no_circuit_comes_for_ends_with_timeout() {
    let mut n = net(&LINES);
    n.circuit(0);
    let id = n.open(18081, false).unwrap();
    n.state.bootstrap = Bootstrap::Cold;
    n.tick(WAIT_SECONDS - 1);
    assert_eq!(n.stream(id), (0, "opening".into()));
    n.tick(1);
    assert_eq!(n.stream(id), (0, "ended 7".into()), "TIMEOUT, as Tor ends one");
    assert!(n.state.streams[0].held.is_none(), "and its open is let go");
}
