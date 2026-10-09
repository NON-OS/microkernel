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

//! net.core's connection and port tables: a client reaches only what it
//! opened, holds at most half of either, gives back what it held when it
//! ends, and a rebuilt stack forgets everything the old one held.

use crate::core_tables::handles::Handles;
use crate::core_tables::ports::Ports;

/// net.core's MAX_SOCKETS and MAX_UDP_SOCKETS, with a number for a handle.
type Conns = Handles<u32, 32>;
type Binds = Ports<u32, 16>;

const A: u32 = 60;
const B: u32 = 61;
const C: u32 = 62;

#[test]
fn a_connection_answers_only_the_client_that_opened_it() {
    let mut t = Conns::new();
    let index = t.alloc(A, 900).expect("a free place");
    assert_ne!(index, 0, "0 is the null handle");
    assert_eq!(t.get(index, A), Some(900));
    assert_eq!(t.get(index, B), None, "B cannot reach A's connection");
    t.free(index, B);
    assert_eq!(t.get(index, A), Some(900), "nor free it");
    t.free(index, A);
    assert_eq!(t.get(index, A), None);
}

#[test]
fn a_port_answers_only_the_client_that_bound_it() {
    let mut t = Binds::new();
    assert!(t.insert(A, 5353, 7));
    assert_eq!(t.get(A, 5353), Some(7));
    assert_eq!(t.get(B, 5353), None, "B cannot reach A's port");
    t.remove(B, 5353);
    assert_eq!(t.get(A, 5353), Some(7), "nor unbind it");
}

/*
 * A rebuilt stack has a new socket set. Kept, the old handles named another
 * client's socket in it, or a socket of the wrong kind, or none, and smoltcp
 * panics on the last two.
 */
#[test]
fn a_rebuilt_stack_forgets_every_connection() {
    let mut t = Conns::new();
    let mut held = Vec::new();
    while let Some(index) = t.alloc([A, B][held.len() & 1], 1) {
        held.push(index);
    }
    assert_eq!(held.len(), 32, "the table is full");
    t.forget_all();
    assert!(held.iter().all(|&i| t.get(i, A).is_none() && t.get(i, B).is_none()));
    assert!(t.alloc(A, 2).is_some(), "the places are free again");
}

#[test]
fn a_rebuilt_stack_forgets_every_port() {
    let mut t = Binds::new();
    let mut port = 1000u16;
    while t.insert([A, B][usize::from(port & 1)], port, 1) {
        port += 1;
    }
    assert_eq!(port, 1016, "the table is full");
    t.forget_all();
    assert!((1000..1016).all(|p| t.get(A, p).is_none() && t.get(B, p).is_none()));
    assert!(t.insert(A, 1000, 2), "the places are free again");
}

#[test]
#[allow(clippy::assertions_on_constants)] // the relation is the point
fn one_client_never_holds_every_connection_or_port() {
    assert!(Conns::PER_OWNER < 32);
    assert!(Binds::PER_OWNER < 16);
}

#[test]
fn a_client_stops_at_half_the_connections_and_another_still_connects() {
    let mut t = Conns::new();
    for _ in 0..Conns::PER_OWNER {
        assert!(t.alloc(A, 1).is_some());
    }
    assert_eq!(t.held_by(A), Conns::PER_OWNER);
    assert_eq!(t.alloc(A, 1), None, "past its half");
    assert!(t.alloc(B, 1).is_some(), "another client still connects");
}

#[test]
fn a_client_stops_at_half_the_ports_and_another_still_binds() {
    let mut t = Binds::new();
    for port in 0..Binds::PER_OWNER as u16 {
        assert!(t.insert(A, 2000 + port, 1));
    }
    assert_eq!(t.held_by(A), Binds::PER_OWNER);
    assert!(!t.insert(A, 3000, 1), "past its half");
    assert!(t.insert(B, 3000, 1), "another client still binds");
}

/*
 * Only the client can close its connection or unbind its port, so one that
 * ended held them for good: two such clients, and no program could connect
 * through net.core again.
 */
#[test]
fn an_ended_client_s_connections_are_taken_out_and_a_living_one_s_kept() {
    let mut t = Conns::new();
    let per = Conns::PER_OWNER as u32;
    let a: Vec<u32> = (0..per).filter_map(|h| t.alloc(A, 100 + h)).collect();
    let b: Vec<u32> = (0..per).filter_map(|h| t.alloc(B, 200 + h)).collect();
    assert_eq!(a.len() + b.len(), 32);
    assert_eq!(t.alloc(C, 1), None, "the table is full");
    let mut gone = t.take_ended(|pid| pid != A);
    gone.sort_unstable();
    assert_eq!(gone, (100..100 + per).collect::<Vec<_>>(), "A's sockets, for release");
    assert!(a.iter().all(|&i| t.get(i, A).is_none()), "A's places are free");
    assert!(b.iter().all(|&i| t.get(i, B).is_some()), "B's stay");
    assert!(t.alloc(C, 1).is_some(), "a new client is served");
    assert!(t.take_ended(|_| true).is_empty(), "nobody ended, nothing taken");
}

#[test]
fn an_ended_client_s_ports_are_taken_out_and_a_living_one_s_kept() {
    let mut t = Binds::new();
    assert!(t.insert(A, 123, 7));
    assert!(t.insert(B, 5353, 8));
    assert_eq!(t.take_ended(|pid| pid != A), vec![7], "A's socket, for unbinding");
    assert_eq!(t.get(A, 123), None);
    assert_eq!(t.get(B, 5353), Some(8), "B's port stays bound");
    assert!(t.take_ended(|_| true).is_empty());
}
