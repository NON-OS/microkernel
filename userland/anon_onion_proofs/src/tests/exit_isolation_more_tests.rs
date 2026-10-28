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

//! Room for a circuit drawn for a port is never made from the warm circuit
//! or from an onion circuit, and an onion circuit takes no stream to a host.

use crate::circuit::Purpose;

use super::exit_circuit::LINES;
use super::exit_net::net;

#[test]
fn an_onion_circuit_takes_no_stream_to_a_host() {
    let mut n = net(&LINES);
    let onion = (n.circuit(0), n.circuit(1)).1;
    n.state.circuits[1].purpose = Purpose::Onion;
    let id = n.open(18081, false).expect("waits");
    assert_eq!(n.stream(id).0, 0, "not the onion circuit, though its exit fits");
    n.tick(1);
    assert!(n.state.circuits.iter().any(|c| c.id == onion), "and room is not made from it");
}

#[test]
fn room_is_never_made_from_the_warm_circuit() {
    let mut n = net(&LINES);
    let held = [n.circuit(0), n.circuit(3), n.circuit(4)];
    for c in n.state.circuits.iter_mut() {
        c.warm = c.id != held[2];
    }
    let id = n.open(18081, false).expect("waits");
    n.tick(1);
    assert!(!n.state.circuits.iter().any(|c| c.id == held[2]), "the idle one went");
    assert!(held[..2].iter().all(|w| n.state.circuits.iter().any(|c| c.id == *w)));
    /* The circuit drawn for it fails, and three warm ones are held. */
    n.state.circuits.retain(|c| c.warm);
    n.circuit(6);
    n.state.circuits.last_mut().unwrap().warm = true;
    n.tick(1);
    assert_eq!(n.state.circuits.len(), 3, "none of them is dropped");
    assert_eq!(n.stream(id).0, 0, "and the stream still waits");
}
