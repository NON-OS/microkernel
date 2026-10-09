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


//! A stream to a .anyone address, from OP_OPEN_STREAM to the service's
//! answer, against the simulated network.

use std::vec::Vec;

use super::world::{net, Faults};
use crate::manager::send_data;

const REQUEST: &[u8] = b"GET /v1/status HTTP/1.0\r\nHost: lander\r\n\r\n";

#[test]
fn a_stream_reaches_the_service_and_carries_v1_status() {
    let mut n = net(Faults::default(), |_| {});
    let host = n.address();
    let id = n.open(host.as_bytes(), 80).expect("the open is taken");
    assert_eq!(n.stream(id).0, "opening", "answered at once, still opening");

    n.run(40);
    assert_eq!(n.stream(id).0, "open", "the service answered CONNECTED");
    assert_eq!(n.world.service_begins, [b":80\0".to_vec()], "one BEGIN, naming no host");
    assert!(n.state.onion.is_empty(), "the lookup is finished");

    assert!(send_data(&mut n.state, id, REQUEST).is_ok(), "the request goes");
    n.exchange();
    assert_eq!(n.world.service_requests, [REQUEST.to_vec()], "the service read the request");
    let (stage, body) = n.stream(id);
    assert_eq!(body, n.world.service_reply, "the answer arrived whole");
    assert_eq!(stage, "ended 6", "and the stream ended cleanly");

    let log = n.log();
    for step in [
        "onion lookup started for stream",
        "onion descriptor checked, introduction points",
        "onion rendezvous point established",
        "onion introduction accepted",
        "onion service reached, stream begin sent",
    ] {
        assert!(log.iter().any(|l| l.contains(step)), "the log names: {step}\n{log:#?}");
    }
}

/* No relay ever sees the address or the service's identity: the HSDir sees
 * the blinded key, the rendezvous point a cookie, the introduction point
 * an encrypted request. */
#[test]
fn no_relay_sees_the_address_or_the_identity() {
    let mut n = net(Faults::default(), |_| {});
    let host = n.address();
    let id = n.open(host.as_bytes(), 80).unwrap();
    n.run(40);
    assert_eq!(n.stream(id).0, "open");
    let address = &host.as_bytes()[..56];
    let identity = n.world.service.identity;
    for body in n.world.relay_saw.iter() {
        assert!(!body.windows(56).any(|w| w.eq_ignore_ascii_case(address)), "a relay saw the address");
        assert!(!body.windows(32).any(|w| w == identity), "a relay saw the identity key");
    }
}

#[test]
fn a_second_stream_reuses_the_rendezvous_circuit() {
    let mut n = net(Faults::default(), |_| {});
    let host = n.address();
    let first = n.open(host.as_bytes(), 80).unwrap();
    n.run(40);
    assert_eq!(n.stream(first).0, "open");
    let asked = n.world.hsdirs_asked.len();
    let introduced = n.world.introduced_at.len();

    let second = n.open(host.as_bytes(), 443).unwrap();
    assert!(n.state.onion.is_empty(), "no new lookup");
    n.exchange();
    assert_eq!(n.stream(second).0, "open");
    assert_eq!(n.world.service_begins.last().unwrap(), &b":443\0".to_vec());
    assert_eq!((n.world.hsdirs_asked.len(), n.world.introduced_at.len()), (asked, introduced));
}

#[test]
fn the_rooted_and_upper_case_forms_reach_the_same_service() {
    let mut n = net(Faults::default(), |_| {});
    let host = std::format!("{}.", n.address().to_ascii_uppercase());
    let id = n.open(host.as_bytes(), 80).unwrap();
    n.run(40);
    assert_eq!(n.stream(id).0, "open");
}

#[test]
fn an_address_that_is_not_valid_is_refused_at_open() {
    let mut n = net(Faults::default(), |_| {});
    let mut host: Vec<u8> = n.address().into_bytes();
    host[3] = if host[3] == b'a' { b'b' } else { b'a' };
    assert_eq!(n.open(&host, 80), Err("bad onion"));
    assert!(n.state.streams.is_empty() && n.state.onion.is_empty());
    assert!(n.state.link.as_ref().unwrap().sent.is_empty(), "nothing went out for it");
}

/* A kept rendezvous circuit takes new streams for CIRCUIT_DIRTY_SECONDS
 * from the moment the service was reached. Past that, an idle one is
 * retired with DESTROY and the next stream runs a fresh lookup, from the
 * cached descriptor. */
#[test]
fn a_kept_rendezvous_circuit_is_retired_after_its_time() {
    let mut n = net(Faults::default(), |_| {});
    let host = n.address();
    let first = n.open(host.as_bytes(), 80).unwrap();
    n.run(40);
    assert_eq!(n.stream(first).0, "open");
    let kept = n.state.circuits.iter().find(|c| c.service.is_some()).map(|c| c.id).expect("kept");
    assert!(send_data(&mut n.state, first, REQUEST).is_ok());
    n.exchange();
    assert_eq!(n.stream(first).0, "ended 6");

    n.pass(599);
    crate::manager::retire_tick(&mut n.state, n.now);
    assert!(n.state.circuits.iter().any(|c| c.id == kept), "still inside its time");

    n.pass(2);
    crate::manager::retire_tick(&mut n.state, n.now);
    n.exchange();
    assert!(!n.state.circuits.iter().any(|c| c.id == kept), "retired");
    assert!(n.world.destroyed.contains(&kept), "with DESTROY");
    let asked = n.world.hsdirs_asked.len();
    let second = n.open(host.as_bytes(), 80).unwrap();
    assert_eq!(n.state.onion.len(), 1, "a fresh lookup");
    n.run(40);
    assert_eq!(n.stream(second).0, "open");
    assert_eq!(n.world.hsdirs_asked.len(), asked, "the descriptor came from the cache");
}
