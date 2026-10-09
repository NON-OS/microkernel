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


//! The descriptor cache, end to end: a second lookup to the same service
//! skips the HSDirs, and cached introduction points that have all gone
//! send the lookup back to them.

use super::world::{net, Faults, Net};
use crate::manager::{retire_tick, send_data};

const REQUEST: &[u8] = b"GET /v1/status HTTP/1.0\r\n\r\n";

/// Connect once, finish the stream, and let the kept circuit be retired.
fn connect_and_retire(n: &mut Net) {
    let first = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(40);
    assert_eq!(n.stream(first).0, "open");
    assert!(send_data(&mut n.state, first, REQUEST).is_ok());
    n.exchange();
    n.pass(601);
    retire_tick(&mut n.state, n.now);
    n.exchange();
    assert!(n.state.circuits.iter().all(|c| c.service.is_none()), "no kept circuit left");
}

#[test]
fn a_second_lookup_uses_the_cached_descriptor() {
    let mut n = net(Faults::default(), |_| {});
    connect_and_retire(&mut n);
    let asked = n.world.hsdirs_asked.len();
    let introduced = n.world.introduced_at.len();

    let second = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(40);
    assert_eq!(n.stream(second).0, "open");
    assert_eq!(n.world.hsdirs_asked.len(), asked, "no HSDir asked: the descriptor came from the cache");
    assert!(n.world.introduced_at.len() > introduced, "a fresh introduction");
}

#[test]
fn a_cached_descriptor_past_its_lifetime_is_fetched_again() {
    let mut n = net(Faults::default(), |_| {});
    connect_and_retire(&mut n);
    /* The descriptor's lifetime is 180 minutes; 601 s have gone. */
    n.pass(180 * 60);
    let asked = n.world.hsdirs_asked.len();
    let second = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(40);
    assert_eq!(n.stream(second).0, "open");
    assert!(n.world.hsdirs_asked.len() > asked, "the HSDirs were asked again");
}

/* The service moved to new introduction points: every cached one refuses,
 * the entry is forgotten, the HSDirs serve the new descriptor, and the
 * stream still opens, on the rendezvous point already established. */
#[test]
fn cached_introduction_points_that_all_fail_send_the_lookup_back_to_the_hsdirs() {
    let mut n = net(Faults::default(), |_| {});
    connect_and_retire(&mut n);
    n.world.rotate();
    let asked = n.world.hsdirs_asked.len();
    let second = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(80);
    assert_eq!(n.stream(second).0, "open");
    assert!(n.world.hsdirs_asked.len() > asked, "the HSDirs were asked again");
    let last = *n.world.introduced_at.last().unwrap();
    assert!(n.world.relays[last].listed, "introduced through one of the new points");
    assert!(n.log().iter().any(|l| l.contains("onion cached introduction points all failed, refetching the descriptor")));
}

/* A lookup that fails leaves nothing in the cache: the next one asks the
 * HSDirs again. */
#[test]
fn a_failed_lookup_caches_nothing() {
    let mut n = net(Faults { bad_signature: true, ..Faults::default() }, |_| {});
    let first = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(80);
    assert_eq!(n.stream(first).0, "ended 2");
    let asked = n.world.hsdirs_asked.len();
    let second = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(80);
    assert_eq!(n.stream(second).0, "ended 2");
    assert!(n.world.hsdirs_asked.len() > asked);
}
