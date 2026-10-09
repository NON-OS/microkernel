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

//! net.nym's session table: one client holds at most half of it, and the
//! sessions of a client that ended are closed while a living client's stay.

use crate::state::table::{Table, TableError, PER_OWNER, TABLE_CAP};
use crate::state::{Gateway, Transport};

const A: u32 = 50;
const B: u32 = 51;
const C: u32 = 52;

fn table() -> Table {
    let mut t = Table::new();
    let gateway = Gateway {
        ip: [10, 0, 0, 1],
        port: 1789,
        stream: 1,
        transport: Transport::RawTcp,
        identity: [0; 32],
        shared_key: [0; 32],
    };
    t.set_gateway(gateway);
    t
}

fn opened(t: &mut Table, owner: u32) -> u32 {
    match t.open(owner, [7; 32]) {
        Ok(id) => id,
        Err(_) => panic!("refused within the share"),
    }
}

fn fill(t: &mut Table, owner: u32) -> Vec<u32> {
    (0..PER_OWNER).map(|_| opened(t, owner)).collect()
}

#[test]
fn a_client_stops_at_half_and_another_still_opens() {
    let mut t = table();
    fill(&mut t, A);
    assert_eq!(t.held_by(A), PER_OWNER);
    assert!(matches!(t.open(A, [7; 32]), Err(TableError::Full)), "past its half");
    assert!(t.open(B, [7; 32]).is_ok(), "another client still opens");
}

/*
 * Only the owner closes a session, so a client that ended kept its sessions,
 * their keys and their unread replies for good, and a directory refresh,
 * which waits for an empty table, never ran again.
 */
#[test]
fn an_ended_client_s_sessions_are_closed_and_its_places_reused() {
    let mut t = table();
    let a = fill(&mut t, A);
    let b = fill(&mut t, B);
    assert!(matches!(t.open(C, [7; 32]), Err(TableError::Full)), "the table is full");
    assert_eq!(t.close_ended(|pid| pid != A), PER_OWNER);
    assert_eq!(t.held_by(A), 0);
    assert!(!t.has_session(A, a[0]), "A's session is gone");
    assert!(b.iter().all(|&id| t.has_session(B, id)), "B's sessions stay");
    assert!(t.open(C, [7; 32]).is_ok(), "a new client is served");
}

#[test]
fn living_clients_keep_their_sessions_and_an_emptied_table_is_idle() {
    let mut t = table();
    let id = opened(&mut t, A);
    assert_eq!(t.close_ended(|_| true), 0);
    assert!(t.has_session(A, id), "A's session stays");
    assert!(!t.idle());
    assert_eq!(t.close_ended(|_| false), 1);
    assert!(t.idle(), "the directory refresh may run again");
}

#[test]
#[allow(clippy::assertions_on_constants)] // the relation is the point
fn one_client_never_holds_every_session() {
    assert!(PER_OWNER < TABLE_CAP);
}
