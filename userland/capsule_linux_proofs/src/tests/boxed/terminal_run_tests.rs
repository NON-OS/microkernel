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

//! A terminal run's box: every box the person can name runs in the one
//! terminal role, as parsed. One naming destinations is granted the
//! anonymity network by the front from the run's argv, so the personality's
//! reading of the box and the front's must agree on which boxes those are.

use crate::boxed::preset::parse;
use crate::boxed::terminal_run::for_terminal;
use linux_net_grant::names_destinations;

const WITH: [&[u8]; 2] = [b"allow:net:10.0.0.1:443", b"allow:/tmp/a,net:example.com:80"];
const WITHOUT: [&[u8]; 4] = [b"strict", b"offline", b"allow:/tmp/a,/home/b", b"lenient"];

/// The argv the kernel keeps for `linux --box <word> prog`.
fn argv(word: &[u8]) -> Vec<u8> {
    [&b"linux\0--box\0"[..], word, b"\0prog"].concat()
}

#[test]
fn every_box_is_as_parsed() {
    for word in WITH.iter().chain(WITHOUT.iter()) {
        assert_eq!(for_terminal(word), parse(word), "{word:?}");
    }
}

#[test]
fn the_front_grants_by_box_exactly_the_boxes_that_name_a_destination() {
    for word in WITH {
        let p = parse(word).expect("a sound box");
        assert!(!p.dests.is_empty() && names_destinations(&argv(word)), "{word:?}");
    }
    for word in WITHOUT.iter().filter(|w| parse(w).is_ok()) {
        let p = parse(word).unwrap();
        assert!(p.dests.is_empty() && !names_destinations(&argv(word)), "{word:?}");
    }
}
