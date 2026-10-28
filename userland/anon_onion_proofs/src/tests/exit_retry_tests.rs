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

//! An exit that ends a stream before it connects, for a reason another exit
//! may not share, gets the open sent once more elsewhere, bytes and all.

use crate::manager::send_data;

use super::exit_circuit::LINES;
use super::exit_far::{BEGIN, CONNECTED, DATA, END};
use super::exit_net::{net, Net};

/// Circuits to relays 1 and 2, both taking 18081, and a stream down the first
/// with a request sent behind its BEGIN: the stream, its circuit, the other.
pub fn opened() -> (Net, u16, u32, u32) {
    let mut n = net(&LINES);
    let (one, two) = (n.circuit(1), n.circuit(2));
    let id = n.open(18081, false).unwrap();
    assert!(send_data(&mut n.state, id, b"POST /json_rpc") == Ok(14));
    let first = n.stream(id).0;
    (n, id, first, if first == one { two } else { one })
}

#[test]
fn an_exitpolicy_end_sends_the_open_and_its_bytes_to_the_other_exit() {
    let (mut n, id, first, other) = opened();
    let sent = n.look();
    assert_eq!(
        sent.iter().map(|s| (s.0, s.1)).collect::<std::vec::Vec<_>>(),
        [(first, BEGIN), (first, DATA)]
    );
    n.answer(first, END, id, &[4]);
    assert_eq!(n.stream(id), (other, "opening".into()), "the caller's stream, moved");
    let again = n.look();
    assert_eq!(again.len(), 2);
    assert_eq!((again[0].0, again[0].1, again[0].2, &again[0].3), (other, BEGIN, id, &sent[0].3));
    assert_eq!((again[1].0, again[1].1, &again[1].3[..]), (other, DATA, &b"POST /json_rpc"[..]));
    let refusing = n.state.circuits.iter().find(|c| c.id == first).unwrap().path[2].clone();
    assert!(n.state.exits.refused.holds(&refusing.rsa_identity, 18081));
    n.answer(other, CONNECTED, id, &[]);
    assert_eq!(n.stream(id), (other, "open".into()));
}
