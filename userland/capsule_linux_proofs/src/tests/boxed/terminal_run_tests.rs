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

//! A terminal run's box, by the role it runs in: the plain terminal role runs
//! every box without a destination and refuses one with, by name; the
//! network role runs only a box with destinations.

use crate::boxed::preset::parse;
use crate::boxed::terminal_run::{for_terminal, NO_LIST, NO_NETWORK};

const WITH: [&[u8]; 2] = [b"allow:net:10.0.0.1:443", b"allow:/tmp/a,net:example.com:80"];
const WITHOUT: [&[u8]; 4] = [b"strict", b"offline", b"allow:/tmp/a,/home/b", b"lenient"];

#[test]
fn a_destination_is_refused_by_name_outside_the_network_role() {
    for word in WITH {
        assert!(parse(word).is_ok(), "the box itself is sound");
        assert_eq!(for_terminal(word, false), Err(NO_NETWORK));
    }
    assert_eq!(NO_NETWORK, "a box naming destinations runs as `linux` with the network role");
}

#[test]
fn the_network_role_runs_only_a_box_with_destinations() {
    for word in WITH {
        assert_eq!(for_terminal(word, true), parse(word));
    }
    for word in WITHOUT.iter().filter(|w| parse(w).is_ok()) {
        assert_eq!(for_terminal(word, true), Err(NO_LIST));
    }
    assert_eq!(NO_LIST, "the network role runs only a box that names destinations");
}

#[test]
fn every_box_without_a_destination_is_as_parsed() {
    for word in WITHOUT {
        assert_eq!(for_terminal(word, false), parse(word));
    }
}
