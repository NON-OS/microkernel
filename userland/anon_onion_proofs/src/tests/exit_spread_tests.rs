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

//! An install's streams over separate circuits (spread_rule.rs): one
//! installer's four streams take four circuits, the warm one first; a fifth
//! joins one; another installer's streams are not counted against it; a
//! stream refused before it connected goes to a circuit its others are not
//! on; and browsing never takes the warm circuit.

use std::collections::BTreeSet;
use std::vec::Vec;

use super::exit_circuit::LINES;
use super::exit_far::END;
use super::exit_net::{net, Net};

/// Four circuits for the web ports, the first warm.
fn four() -> (Net, Vec<u32>) {
    let mut n = net(&LINES);
    let ids: Vec<u32> = [0, 3, 4, 6].into_iter().map(|exit| n.circuit(exit)).collect();
    n.state.circuits[0].warm = true;
    (n, ids)
}

#[test]
fn four_streams_of_one_installer_take_four_circuits_warm_first() {
    let (mut n, ids) = four();
    let streams: Vec<u16> = (0..4).map(|_| n.open_for(443, true, 41).expect("opened")).collect();
    let on: Vec<u32> = streams.iter().map(|s| n.stream(*s).0).collect();
    assert_eq!(on[0], ids[0], "the warm circuit first");
    assert_eq!(on.iter().collect::<BTreeSet<_>>().len(), 4, "four circuits: {on:?}");
    let fifth = n.open_for(443, true, 41).expect("opened");
    assert!(on.contains(&n.stream(fifth).0), "a fifth joins one of them");
}

#[test]
fn another_installers_streams_do_not_count_against_it() {
    let (mut n, ids) = four();
    let a = n.open_for(443, true, 41).expect("opened");
    let b = n.open_for(443, true, 42).expect("opened");
    assert_eq!((n.stream(a).0, n.stream(b).0), (ids[0], ids[0]), "each installer's first takes the warm one");
}

#[test]
fn browsing_never_takes_the_warm_circuit_beside_an_installs_streams() {
    let (mut n, ids) = four();
    for _ in 0..3 {
        n.open_for(443, true, 41).expect("opened");
    }
    for _ in 0..6 {
        let s = n.open(443, false).expect("opened");
        assert_ne!(n.stream(s).0, ids[0]);
    }
}

#[test]
fn a_stream_refused_before_it_connected_goes_where_its_others_are_not() {
    let mut n = net(&LINES);
    let ids: Vec<u32> = [0, 3, 4, 6, 7].into_iter().map(|exit| n.circuit(exit)).collect();
    n.state.circuits[0].warm = true;
    let streams: Vec<u16> = (0..3).map(|_| n.open_for(443, true, 41).expect("opened")).collect();
    let before: Vec<u32> = streams.iter().map(|s| n.stream(*s).0).collect();
    let broken = streams[1];
    n.answer(before[1], END, broken, &[4]);
    let after = n.stream(broken).0;
    assert!(after != 0, "sent to another exit at once");
    assert!(!before.contains(&after) || after == before[1], "not onto a sibling's circuit: {before:?} -> {after}");
    assert!(ids.contains(&after));
}
