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

//! What is not sent again: a stream already connected, or one that sent more
//! before it connected than is kept; and a refusing exit is not drawn again.

use crate::manager::send_data;

use super::exit_circuit::LINES;
use super::exit_far::{BEGIN, CONNECTED, END};
use super::exit_net::net;

#[test]
fn an_end_after_connected_is_passed_back_and_blames_no_exit_for_the_port() {
    let mut n = net(&LINES);
    let c = n.circuit(1);
    let id = n.open(18081, false).unwrap();
    n.answer(c, CONNECTED, id, &[93, 184, 216, 34, 0, 0, 0, 60]);
    n.look();
    n.answer(c, END, id, &[4]);
    assert_eq!(n.stream(id), (c, "ended 4".into()));
    assert!(n.look().is_empty());
    assert!(!n.state.exits.refused.holds(&n.state.relays[1].rsa_identity, 18081));
}

#[test]
fn a_stream_that_sent_past_what_is_kept_is_not_sent_again() {
    let mut n = net(&LINES);
    n.circuit(1);
    n.circuit(2);
    let id = n.open(18081, false).unwrap();
    let c = n.stream(id).0;
    /* One byte past EARLY_MAX, the 8 KiB kept behind an open. */
    let big = std::vec![7u8; 8 * 1024 + 1];
    assert!(send_data(&mut n.state, id, &big) == Ok(big.len()));
    n.answer(c, END, id, &[4]);
    assert_eq!(n.stream(id), (c, "ended 4".into()), "the bytes could not be sent again");
}

#[test]
fn with_no_other_circuit_it_waits_for_one_not_ending_at_the_refusing_exit() {
    let mut n = net(&LINES);
    let (_web, c) = (n.circuit(0), n.circuit(1));
    let id = n.open(18081, false).unwrap();
    n.answer(c, END, id, &[4]);
    assert_eq!(n.stream(id), (0, "opening".into()), "waiting, not ended");
    n.tick(1);
    let drawn = n.state.circuits.last().unwrap().path[2].clone();
    assert_ne!(
        drawn.rsa_identity, n.state.relays[1].rsa_identity,
        "the refusing exit is passed over"
    );
    assert!(n.state.exits.takes(&drawn, 18081));
    let up = n.bring_up();
    n.look();
    n.tick(0);
    assert_eq!(n.stream(id), (up, "opening".into()));
    assert!(n.look().iter().any(|s| s.0 == up && s.1 == BEGIN && s.2 == id));
}
