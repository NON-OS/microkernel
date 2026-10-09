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


//! A lookup that cannot succeed ends its stream with the right reason and
//! leaves nothing behind.

use super::world::{net, Faults, Net};
use crate::circuit::CircuitStage;
use crate::manager::retire_tick;

fn nothing_left(n: &mut Net) {
    assert!(n.state.onion.is_empty(), "no lookup left running");
    retire_tick(&mut n.state, n.now);
    assert!(
        n.state.circuits.iter().all(|c| c.stage == CircuitStage::Open && c.service.is_some() || c.purpose == crate::circuit::Purpose::General),
        "no onion circuit of the failed lookup left in the table"
    );
}

#[test]
fn a_descriptor_with_a_bad_signature_is_refused_everywhere() {
    let mut n = net(Faults { bad_signature: true, ..Faults::default() }, |_| {});
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(80);
    assert_eq!(n.stream(id).0, "ended 2");
    assert!(n.log().iter().any(|l| l.contains("onion descriptor refused, signature")));
    assert!(n.world.introduced_at.is_empty(), "nothing was introduced on a forged descriptor");
    nothing_left(&mut n);
}

#[test]
fn a_service_behind_client_authorization_is_named_and_not_reached() {
    let mut n = net(Faults { client_auth: true, ..Faults::default() }, |_| {});
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(80);
    assert_eq!(n.stream(id).0, "ended 2");
    assert!(n.log().iter().any(|l| l.contains("onion service needs client authorization")));
    nothing_left(&mut n);
}

#[test]
fn a_service_that_never_comes_runs_the_lookup_out_of_time() {
    let mut n = net(Faults::default(), |_| {});
    n.world.meets = false;
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    for _ in 0..60 {
        n.run(2);
        n.pass(1);
    }
    n.run(2);
    assert_eq!(n.stream(id).0, "ended 7");
    let log = n.log();
    assert!(log.iter().any(|l| l.contains("onion service did not reach the rendezvous point in time")));
    assert!(log.iter().any(|l| l.contains("onion lookup ran out of time")));
    assert!(n.world.introduced_at.len() >= 2, "it went on to the next introduction point meanwhile");
    nothing_left(&mut n);
}

#[test]
fn a_rendezvous_handshake_that_does_not_verify_ends_the_stream() {
    let mut n = net(Faults::default(), |_| {});
    n.world.bad_rendezvous_mac = true;
    let id = n.open(n.address().as_bytes(), 80).unwrap();
    n.run(80);
    assert_eq!(n.stream(id).0, "ended 1");
    assert!(n.log().iter().any(|l| l.contains("onion rendezvous handshake did not verify")));
    assert!(n.world.service_begins.is_empty(), "no BEGIN on an unverified hop");
    nothing_left(&mut n);
}
