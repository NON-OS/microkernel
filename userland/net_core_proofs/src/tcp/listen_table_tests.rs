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

use super::listeners::table::{Listeners, Refusal, LISTENER_BASE};

type T = Listeners<u16, 2, 3>;

#[test]
fn listener_handles_never_meet_connection_handles() {
    for conn in 0..=1024u32 {
        assert!(!T::is_listener(conn), "{conn}");
    }
    assert!(!T::is_listener(LISTENER_BASE));
    assert!(T::is_listener(LISTENER_BASE + 1));
    assert!(T::is_listener(LISTENER_BASE + 2));
    assert!(!T::is_listener(LISTENER_BASE + 3), "past the table");
    assert!(!T::is_listener(u32::MAX));
}

#[test]
fn one_listener_a_port_and_a_bounded_table() {
    let mut t = T::new();
    let a = t.open(7, 2222, 100).unwrap();
    assert_eq!(a, LISTENER_BASE + 1);
    assert_eq!(t.open(8, 2222, 101), Err(Refusal::PortInUse), "whoever asks");
    assert_eq!(t.open(7, 80, 102), Ok(LISTENER_BASE + 2));
    assert_eq!(t.open(7, 81, 103), Err(Refusal::Full));
}

#[test]
fn only_the_owner_reaches_or_closes_a_listener() {
    let mut t = T::new();
    let a = t.open(7, 2222, 100).unwrap();
    assert!(t.get_mut(a, 8).is_none());
    assert_eq!(t.close(a, 8), None);
    assert!(t.get_mut(a, 7).is_some());
    assert!(t.get_mut(a + 1, 7).is_none(), "an empty slot");
    assert!(t.get_mut(5, 7).is_none(), "a connection handle");
    assert!(t.get_mut(a, 7).unwrap().push(200));
    assert_eq!(t.close(a, 7), Some(vec![100, 200]), "its socket and its queue");
    assert_eq!(t.close(a, 7), None, "once");
    assert!(t.open(9, 2222, 104).is_ok(), "the port is free again");
}

#[test]
fn the_backlog_holds_its_bound_and_hands_out_the_oldest_ready() {
    let mut t = T::new();
    let a = t.open(7, 2222, 100).unwrap();
    let l = t.get_mut(a, 7).unwrap();
    assert!(l.push(1) && l.push(2) && l.push(3));
    assert!(!l.has_room());
    assert!(!l.push(4), "a full backlog takes no more");
    assert_eq!(l.take(|h| h != 1), Some(2), "1 is still in its handshake");
    assert_eq!(l.take(|_| true), Some(1), "order kept");
    assert!(l.restore(1), "a refused hand-out goes back first");
    assert_eq!(l.take(|_| true), Some(1));
    assert_eq!(l.take(|_| true), Some(3));
    assert_eq!(l.take(|_| true), None);
}

#[test]
fn restore_never_overfills() {
    let mut t = T::new();
    let a = t.open(7, 2222, 100).unwrap();
    let l = t.get_mut(a, 7).unwrap();
    assert!(l.push(1) && l.push(2) && l.push(3));
    assert!(!l.restore(9));
    assert_eq!(l.all_sockets(), vec![100, 1, 2, 3]);
}

#[test]
fn closed_queued_connections_leave_and_the_rest_keep_order() {
    let mut t = T::new();
    let a = t.open(7, 2222, 100).unwrap();
    let l = t.get_mut(a, 7).unwrap();
    assert!(l.push(1) && l.push(2) && l.push(3));
    assert_eq!(l.drop_queued(|h| h != 2), vec![2]);
    assert!(l.has_room());
    assert_eq!(l.all_sockets(), vec![100, 1, 3]);
}

#[test]
fn an_ended_owner_or_a_route_change_takes_every_socket() {
    let mut t = T::new();
    let a = t.open(7, 2222, 100).unwrap();
    t.open(8, 80, 101).unwrap();
    assert!(t.get_mut(a, 7).unwrap().push(1));
    assert_eq!(t.take_ended(|pid| pid != 7), vec![100, 1]);
    assert!(t.any());
    assert_eq!(t.take_all(), vec![101]);
    assert!(!t.any());
}

#[test]
fn a_rebuilt_stack_forgets_every_listener() {
    let mut t = T::new();
    let a = t.open(7, 2222, 100).unwrap();
    t.forget_all();
    assert!(!t.any());
    assert!(t.get_mut(a, 7).is_none());
    assert!(t.open(7, 2222, 101).is_ok());
}
