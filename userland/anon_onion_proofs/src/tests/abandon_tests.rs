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


//! A caller that gives up, a link that goes, and the limits.

use super::world::{net, Faults};
use crate::manager::send_data;

#[test]
fn a_stream_closed_mid_lookup_takes_its_circuits_with_it() {
    let mut n = net(Faults::default(), |_| {});
    n.world.meets = false;
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(30);
    assert_eq!(n.stream(id).0, "opening", "introduced, waiting for the service");
    let held: std::vec::Vec<u32> = n.state.circuits.iter().map(|c| c.id).collect();
    assert!(!held.is_empty());

    /* What close_stream does to a stream with no circuit yet. */
    n.state.streams.retain(|s| s.id != id);
    n.run(1);
    assert!(n.state.onion.is_empty(), "the lookup is gone");
    assert!(n.state.circuits.is_empty(), "its circuits are gone");
    for c in held {
        assert!(n.world.destroyed.contains(&c), "circuit {c} was destroyed at the relays");
    }
    assert!(n.log().iter().any(|l| l.contains("onion lookup abandoned with its stream")));
}

#[test]
fn a_lost_link_ends_the_lookup() {
    let mut n = net(Faults::default(), |_| {});
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(3);
    /* What link_lost::lost does. */
    n.state.link = None;
    n.state.circuits.clear();
    n.state.streams.clear();
    crate::manager::onion_tick(&mut n.state, n.now);
    assert!(n.state.onion.is_empty());
    assert_eq!(n.stream(id).0, "gone");
}

#[test]
fn bytes_on_a_stream_still_opening_wait_with_the_caller() {
    let mut n = net(Faults::default(), |_| {});
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    assert!(matches!(send_data(&mut n.state, id, b"early"), Err(crate::manager::SendError::WouldBlock)));
    n.run(40);
    assert_eq!(n.stream(id).0, "open");
    assert!(send_data(&mut n.state, id, b"now").is_ok());
}

#[test]
fn four_lookups_at_once_and_no_more() {
    let mut n = net(Faults::default(), |_| {});
    let host = n.address();
    for _ in 0..4 {
        assert!(n.open(host.as_bytes(), 80).is_ok());
    }
    assert_eq!(n.open(host.as_bytes(), 80), Err("table full"));
    n.run(60);
    /* The first to finish leaves a kept circuit, so the rest open on it. */
    let open = n.state.streams.iter().filter(|s| s.stage == crate::stream::StreamStage::Open).count();
    assert_eq!(open, 4, "all four reach the service");
    assert!(n.state.onion.is_empty());
}
