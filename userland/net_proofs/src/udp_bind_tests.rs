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

//! net.udp's ports: one client's share of them, and the ports of clients
//! that ended without unbinding.

use crate::udp_binds::bind::BindEntry;
use crate::udp_binds::table::{BindTable, TableError, BINDS_PER_PID, MAX_BINDS};

fn bind(t: &mut BindTable, pid: u32, port: u16) -> Result<(), TableError> {
    t.insert(BindEntry::new(pid, port))
}

#[test]
#[allow(clippy::assertions_on_constants)] // the relation is the point
fn one_client_never_holds_every_port() {
    assert!(BINDS_PER_PID < MAX_BINDS);
}

#[test]
fn a_client_stops_at_its_share_and_others_still_bind() {
    let mut t = BindTable::new();
    for port in 0..BINDS_PER_PID as u16 {
        assert!(bind(&mut t, 7, 40_000 + port).is_ok());
    }
    assert!(bind(&mut t, 7, 50_000) == Err(TableError::Full), "past its share");
    assert!(bind(&mut t, 8, 50_000).is_ok(), "another client still binds");
}

/*
 * A client that ended without unbinding kept its port for good, so the NTP
 * client coming back could not bind 123 again.
 */
#[test]
fn an_ended_client_s_port_can_be_bound_again() {
    let mut t = BindTable::new();
    assert!(bind(&mut t, 30, 123).is_ok());
    assert!(bind(&mut t, 31, 123) == Err(TableError::InUse));
    assert_eq!(t.take_dead(|pid| pid != 30), 1);
    assert!(bind(&mut t, 31, 123).is_ok(), "the service that came back has its port");
}

#[test]
fn ended_clients_free_a_full_table_and_living_ones_keep_theirs() {
    let mut t = BindTable::new();
    for i in 0..MAX_BINDS as u16 {
        let pid = if i % 2 == 0 { 40 } else { 41 };
        assert!(bind(&mut t, pid, 10_000 + i).is_ok());
    }
    assert!(bind(&mut t, 42, 9_999) == Err(TableError::Full));
    assert_eq!(t.take_dead(|pid| pid != 40), MAX_BINDS / 2);
    assert!(t.find_owned_mut(41, 10_001).is_some(), "a living client keeps its port");
    assert!(bind(&mut t, 42, 9_999).is_ok());
}

#[test]
fn nothing_is_freed_while_every_owner_runs() {
    let mut t = BindTable::new();
    assert!(bind(&mut t, 50, 53).is_ok());
    assert_eq!(t.take_dead(|_| true), 0);
    assert!(t.find_owned_mut(50, 53).is_some());
}
