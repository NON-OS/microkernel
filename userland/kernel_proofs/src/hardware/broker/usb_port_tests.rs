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

//! A USB host's port facts on the wire: what the capsule encodes the
//! kernel decodes the same, and every record that says more than its
//! presence bits allow, or uses a reserved value, is refused whole.

use super::acpi::usb_port_wire::*;

fn port(port: u8, flags: u8, connector: u8, panel: u8) -> PortFact {
    PortFact { parent: 0, port, flags, connector, panel }
}

#[test]
fn internal_and_external_ports_round_trip() {
    let camera = port(4, PLD | LID | 1 << REACH_SHIFT, 0, 0);
    let type_a = port(1, UPC | CONNECTABLE | PLD | VISIBLE | 2 << REACH_SHIFT, 3, 3);
    let hub = PortFact { parent: 5, port: 1, flags: 0, connector: 0, panel: 0 };
    let ports = [type_a, camera, hub];
    let b = encode(&ports).expect("encodes");
    assert_eq!(b.len(), 3 * PORT_LEN);
    assert_eq!(decode(&b).unwrap(), ports);
    assert_eq!(decode(&b).unwrap()[1].reach(), 1);
    assert_eq!(decode(&b).unwrap()[0].reach(), 2);
}

#[test]
fn nothing_published_is_an_empty_host() {
    assert_eq!(encode(&[]), Some(vec![]));
    assert_eq!(decode(&[]), Some(vec![]));
}

#[test]
fn lengths_off_the_record_are_refused() {
    let b = encode(&[port(1, 0, 0, 0)]).unwrap();
    assert_eq!(decode(&b[..7]), None);
    assert_eq!(decode(&[b.clone(), vec![0]].concat()), None);
    let most = vec![0u8; MAX_PORTS * PORT_LEN];
    assert_eq!(decode(&most).map(|p| p.len()), Some(MAX_PORTS));
    assert_eq!(decode(&vec![0u8; (MAX_PORTS + 1) * PORT_LEN]), None);
    assert_eq!(encode(&vec![port(1, 0, 0, 0); MAX_PORTS + 1]), None);
}

#[test]
fn facts_without_their_object_are_refused() {
    assert_eq!(encode(&[port(1, CONNECTABLE, 0, 0)]), None);
    for f in [VISIBLE, LID, DOCK] {
        assert_eq!(encode(&[port(1, f, 0, 0)]), None);
        assert!(encode(&[port(1, f | PLD, 0, 0)]).is_some());
    }
}

#[test]
fn reserved_values_are_refused() {
    assert_eq!(encode(&[port(1, 3 << REACH_SHIFT, 0, 0)]), None);
    assert_eq!(encode(&[port(1, PLD, 0, 7)]), None);
    assert!(encode(&[port(1, PLD, 0, 6)]).is_some());
    let mut b = encode(&[port(1, 0, 0, 0)]).unwrap();
    for at in 5..8 {
        b[at] = 1;
        assert_eq!(decode(&b), None);
        b[at] = 0;
    }
}
