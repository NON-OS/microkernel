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


//! A lookup that meets HSDirs without the descriptor, or introduction
//! points that refuse, moves on to the next and still connects.

use std::vec::Vec;

use super::world::{net, Faults, SimRelay};

/// Only one responsible HSDir holds the descriptor.
fn one_holder(relays: &mut [SimRelay]) {
    let holders: Vec<usize> = relays.iter().enumerate().filter(|(_, r)| r.holds).map(|(i, _)| i).collect();
    assert_eq!(holders.len(), 6, "two replicas, three HSDirs each");
    for at in holders.iter().skip(1) {
        relays[*at].holds = false;
    }
}

#[test]
fn hsdirs_without_the_descriptor_are_passed_over() {
    let mut n = net(Faults::default(), |relays| one_holder(relays));
    let holder = n.world.relays.iter().position(|r| r.holds).unwrap();
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(80);
    assert_eq!(n.stream(id).0, "open");
    let asked = &n.world.hsdirs_asked;
    assert_eq!(asked.last(), Some(&holder), "the last HSDir asked held it");
    assert!(asked[..asked.len() - 1].iter().all(|a| !n.world.relays[*a].holds), "each earlier one answered 404");
    assert!(asked.len() > 1, "with this seed the holder is not the first asked: {asked:?}");
    assert!(n.log().iter().any(|l| l.contains("onion hsdir holds no descriptor")));
}

#[test]
fn no_hsdir_holding_it_ends_the_stream_with_resolvefailed() {
    let mut n = net(Faults::default(), |relays| relays.iter_mut().for_each(|r| r.holds = false));
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(80);
    assert_eq!(n.stream(id).0, "ended 2");
    let mut asked = n.world.hsdirs_asked.clone();
    asked.sort();
    asked.dedup();
    assert_eq!(asked.len(), 6, "every responsible HSDir was asked, each once");
    assert!(n.state.onion.is_empty());
}

#[test]
fn refusing_introduction_points_are_passed_over() {
    let mut n = net(Faults::default(), |relays| {
        for r in relays.iter_mut().filter(|r| matches!(r.intro_index, Some(0) | Some(1))) {
            r.refuse = Some(2);
        }
    });
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(80);
    assert_eq!(n.stream(id).0, "open");
    let last = *n.world.introduced_at.last().unwrap();
    assert_eq!(n.world.relays[last].intro_index, Some(2), "the one that accepts carried it");
    assert!(n.world.introduced_at.iter().filter(|a| **a == last).count() == 1);
}

#[test]
fn every_introduction_point_refusing_ends_the_stream() {
    let mut n = net(Faults::default(), |relays| {
        for r in relays.iter_mut().filter(|r| r.intro_index.is_some()) {
            r.refuse = Some(1);
        }
    });
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(80);
    assert_eq!(n.stream(id).0, "ended 3");
    assert_eq!(n.world.introduced_at.len(), 3, "each introduction point tried once");
    assert!(n.log().iter().any(|l| l.contains("onion introduction refused")));
}

#[test]
fn an_hsdir_answering_something_else_is_passed_over() {
    let mut n = net(Faults::default(), |relays| {
        let holders: Vec<usize> = relays.iter().enumerate().filter(|(_, r)| r.holds).map(|(i, _)| i).collect();
        for at in holders.iter().skip(1) {
            relays[*at].holds = false;
            relays[*at].garbage = true;
        }
    });
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(80);
    assert_eq!(n.stream(id).0, "open");
    let asked = &n.world.hsdirs_asked;
    assert!(asked[..asked.len() - 1].iter().all(|a| n.world.relays[*a].garbage));
    assert!(asked.len() > 1, "with this seed a garbage HSDir is asked first: {asked:?}");
    assert!(n.log().iter().any(|l| l.contains("onion descriptor refused")));
}

/* An introduction point whose certificate does not verify is never used:
 * the descriptor's other points carry the stream. */
#[test]
fn an_introduction_point_with_a_forged_certificate_is_never_used() {
    let mut n = net(Faults { bad_first_intro_cert: true, ..Faults::default() }, |_| {});
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(80);
    assert_eq!(n.stream(id).0, "open");
    assert!(n.world.introduced_at.iter().all(|a| n.world.relays[*a].intro_index != Some(0)));
    assert!(n.log().iter().any(|l| l.contains("onion descriptor checked, introduction points 2")));
}
