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

//! The isolation net.anon keeps is unchanged by the choice by port: the warm
//! circuit takes installs and nothing else, and a stream keeps its id and its
//! owner when it is sent again.

use crate::stream::owned;

use super::exit_circuit::LINES;
use super::exit_far::END;
use super::exit_net::{net, Net};

/// A warm circuit to relay 1, which takes 18081, beside one to relay 0.
fn warm_beside_web() -> (Net, u32, u32) {
    let mut n = net(&LINES);
    let (web, warm) = (n.circuit(0), n.circuit(1));
    n.state.circuits[1].warm = true;
    (n, web, warm)
}

#[test]
fn a_browsing_stream_never_takes_the_warm_circuit_whatever_its_exit() {
    let (mut n, _web, warm) = warm_beside_web();
    let id = n.open(18081, false).expect("waits");
    assert_eq!(n.stream(id), (0, "opening".into()), "not the warm circuit, though it fits");
    n.tick(1);
    let drawn = n.state.circuits.last().unwrap();
    assert!(drawn.id != warm && !drawn.warm, "a circuit of its own, not marked warm");
    assert!(n.state.exits.takes(&drawn.path[2], 18081));
}

#[test]
fn an_install_still_takes_the_warm_circuit_first_and_others_do_not() {
    let (mut n, web, warm) = warm_beside_web();
    let install = n.open(443, true).unwrap();
    let browse = n.open(443, false).unwrap();
    assert_eq!((n.stream(install).0, n.stream(browse).0), (warm, web));
}

#[test]
fn a_browsing_stream_sent_again_never_lands_on_the_warm_circuit() {
    let mut n = net(&LINES);
    let (first, warm) = (n.circuit(1), n.circuit(2));
    n.state.circuits[1].warm = true;
    let id = n.open(18081, false).unwrap();
    assert_eq!(n.stream(id).0, first);
    n.answer(first, END, id, &[4]);
    assert_eq!(n.stream(id), (0, "opening".into()), "it waits; the warm one is not taken");
    assert!(n.state.circuits.iter().any(|c| c.id == warm && c.warm));
    assert!(owned(&n.state.streams, 7, id).is_some(), "the same id, for the same caller");
    assert!(owned(&n.state.streams, 8, id).is_none(), "and for no other");
}
