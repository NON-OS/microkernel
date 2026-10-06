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

//! A lookup asks every server the lease named and takes the first address;
//! it fails only once every server asked has failed.

use crate::dns_pending::{Look, Table, Waiting};
use crate::dns_verdict::{combine, Found};

const IP: [u8; 4] = [93, 184, 216, 34];

#[test]
fn the_first_address_wins_whatever_the_others_say() {
    assert_eq!(combine([Found::Failed, Found::Address(IP), Found::Pending]), Found::Address(IP));
    assert_eq!(combine([Found::Pending, Found::Pending, Found::Address(IP)]), Found::Address(IP));
}

// The router's resolver still starting after a reboot fails its query; the
// second server named by the lease is still asked and may answer.
#[test]
fn one_failed_server_leaves_the_lookup_waiting_on_the_others() {
    assert_eq!(combine([Found::Failed, Found::Pending, Found::Failed]), Found::Pending);
}

#[test]
fn every_server_failed_fails_the_lookup() {
    assert_eq!(combine([Found::Failed, Found::Failed, Found::Failed]), Found::Failed);
}

// A lease with no DNS server starts no query at all.
#[test]
fn no_server_asked_fails_the_lookup() {
    assert_eq!(combine([]), Found::Failed);
}

// A query whose result was read is noted in the lookup and stays noted on the
// next sweep: smoltcp frees a read query's slot and may give it to another
// lookup, so reading it again would take that lookup's answer.
#[test]
fn what_a_sweep_notes_in_a_lookup_is_kept() {
    let mut t = Table::new();
    let unread = [true, true, true];
    t.add(Waiting { pid: 9, request_id: 1, query: unread, deadline_ms: 3_000 }).unwrap();
    t.sweep(0, |w, _| {
        w.query[0] = false;
        Look::Pending
    });
    t.sweep(1, |w, _| {
        assert_eq!(w.query, [false, true, true]);
        Look::Answered
    });
    assert!(!t.any());
}
