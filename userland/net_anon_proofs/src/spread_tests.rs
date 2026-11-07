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

//! An install's streams spread over circuits: each of up to four takes a
//! circuit none of its others is on while one is open, the warm one first;
//! only when every circuit carries one does a stream join another. Every
//! other stream is ordered as before and never takes the warm circuit.

use crate::spread_rule::rank;

/// The circuit a stream takes of `circuits` (warm, proven), given which
/// already carry a stream of the same caller: the lowest rank, first in
/// order among equals, as pick_circuit.rs scans them.
fn take(install: bool, circuits: &[(bool, bool)], mine: &[bool]) -> Option<usize> {
    let ranked: Vec<Option<u8>> =
        circuits.iter().zip(mine).map(|(&(w, p), &m)| rank(install, w, p, install && m)).collect();
    let best = ranked.iter().flatten().min()?;
    ranked.iter().position(|r| r.as_ref() == Some(best))
}

const FOUR: [(bool, bool); 4] = [(true, true), (false, true), (false, true), (false, false)];

#[test]
fn four_install_streams_take_four_circuits() {
    let mut mine = [false; 4];
    let mut order = Vec::new();
    for _ in 0..4 {
        let at = take(true, &FOUR, &mine).expect("a circuit");
        assert!(!mine[at], "a circuit already carrying one of its streams");
        mine[at] = true;
        order.push(at);
    }
    assert_eq!(order, [0, 1, 2, 3], "warm, then delivered, then the rest");
}

#[test]
fn a_fifth_stream_joins_in_the_same_order() {
    assert_eq!(take(true, &FOUR, &[true; 4]), Some(0));
    assert_eq!(take(true, &FOUR, &[true, false, true, true]), Some(1));
}

#[test]
fn a_stream_whose_circuit_broke_goes_to_one_its_others_are_not_on() {
    // Streams on 0, 1 and 2; 1 broke and its stream ends there: it is asked again.
    let circuits = [(true, true), (false, true), (false, true), (false, true)];
    assert_eq!(take(true, &circuits, &[true, false, true, true]), Some(1));
    // Circuit 1 gone from the list (retired): the stream takes one free of its others.
    let left = [(true, true), (false, true), (false, true), (false, false)];
    assert_eq!(take(true, &left, &[true, false, true, false]), Some(1));
}

#[test]
fn other_streams_are_ordered_as_before_and_never_take_the_warm_circuit() {
    for warm in [false, true] {
        for proven in [false, true] {
            for mine in [false, true] {
                let r = rank(false, warm, proven, mine);
                assert_eq!(r.is_none(), warm, "warm is for installs only");
                if !warm {
                    assert_eq!(r, Some(if proven { 1 } else { 2 }), "the caller's others do not count");
                }
            }
        }
    }
    assert_eq!(take(false, &FOUR, &[false; 4]), Some(1));
}

#[test]
fn an_installs_ranks_put_every_free_circuit_before_any_shared_one() {
    for w in [false, true] {
        for p in [false, true] {
            let free = rank(true, w, p, false).expect("free");
            for w2 in [false, true] {
                for p2 in [false, true] {
                    assert!(free < rank(true, w2, p2, true).expect("shared"));
                }
            }
        }
    }
}
