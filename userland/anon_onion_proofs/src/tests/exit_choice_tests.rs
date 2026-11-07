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

//! A stream goes down a circuit whose exit takes its port; for the web ports
//! the choice is the one it always was.

use std::collections::BTreeSet;

use crate::directory::consensus::Entry;

use super::exit_circuit::LINES;
use super::exit_far::BEGIN;
use super::exit_net::net;

#[test]
fn a_stream_to_a_port_goes_down_a_circuit_whose_exit_takes_it() {
    let mut n = net(&LINES);
    let (_web, wallet) = (n.circuit(0), n.circuit(1));
    for port in [18081, 18089, 18081, 18080] {
        let id = n.open(port, false).expect("opened");
        assert_eq!(n.stream(id), (wallet, "opening".into()), "port {port}");
    }
    let seen = n.look();
    assert_eq!(seen.len(), 4);
    assert!(seen.iter().all(|(circuit, command, _, _)| (*circuit, *command) == (wallet, BEGIN)));
    assert!(seen[0].3.starts_with(b"node.example:18081\0"), "the BEGIN names the port");
}

#[test]
fn the_web_ports_go_down_every_web_circuit_as_before() {
    let mut n = net(&LINES);
    let (a, b, no_web) = (n.circuit(0), n.circuit(1), n.circuit(5));
    let mut used = BTreeSet::new();
    for port in [443, 80, 443, 80, 443, 443] {
        let id = n.open(port, false).expect("opened");
        used.insert(n.stream(id).0);
    }
    assert_eq!(used, BTreeSet::from([a, b]), "spread over both, as the cursor always did");
    assert!(!used.contains(&no_web), "an exit without both web ports takes no web stream");
}

#[test]
fn a_port_no_exit_lets_out_or_a_table_with_no_circuit_is_refused_at_once() {
    let mut n = net(&LINES);
    assert!(n.open(443, false) == Err("no circuit"), "no circuit yet, as before");
    assert!(n.open(18081, false) == Err("no circuit"));
    assert!(n.open(25, false) == Err("no exit"), "no exit lets 25 out, circuit or none");
    n.circuit(0);
    assert!(n.open(25, false) == Err("no exit"), "no exit lets 25 out");
    assert!(n.open(5223, false).is_ok(), "an exit lets 5223 out: it waits for one");
    assert!(n.look().is_empty());
}

#[test]
fn a_port_every_exit_refused_this_boot_is_no_exit_and_others_still_wait() {
    let mut n = net(&LINES);
    n.circuit(0);
    let state = &n.state;
    let ids: std::vec::Vec<_> = state
        .relays
        .iter()
        .filter(|r| state.exits.ports.lets_out(r, 18081))
        .map(|r| r.rsa_identity)
        .collect();
    assert_eq!(ids.len(), 3, "three exits let 18081 out, one by a reject list");
    for id in ids {
        n.state.exits.refused.note(id, 18081);
    }
    assert!(n.open(18081, false) == Err("no exit"), "every exit that lets it out refused it");
    assert!(n.open(443, false).is_ok(), "another port is untouched");
}

#[test]
fn a_port_no_exit_in_hand_lets_out_waits_while_the_directory_is_not_whole() {
    let mut n = net(&LINES);
    n.circuit(0);
    let entry = Entry {
        rsa_identity: [9; 20],
        address: [192, 0, 2, 9],
        or_port: 9001,
        microdesc_digest: [9; 32],
        flags: Default::default(),
        weight: 1,
    };
    n.state.entries = std::vec![entry; 2];
    n.state.dir.micro = 0;
    assert!(n.open(25, false) == Err("no circuit"), "a batch not yet asked may hold an exit");
    n.state.dir.micro = 1;
    assert!(n.open(25, false) == Err("no exit"), "every batch asked: said at once");
    n.state.refreshing = true;
    assert!(n.open(25, false) == Err("no circuit"), "a refresh may bring one");
    assert!(n.state.streams.is_empty());
}
