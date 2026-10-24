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

//! A terminal run's box: everything a box may be, but no destination, which
//! is refused by name rather than promised and never reached.

use crate::boxed::preset::parse;
use crate::boxed::terminal_run::{for_terminal, NO_NETWORK};

#[test]
fn a_destination_is_refused_by_name_for_a_terminal_run() {
    for word in [&b"allow:net:10.0.0.1:443"[..], b"allow:/tmp/a,net:example.com:80"] {
        assert!(parse(word).is_ok(), "the box itself is sound");
        assert_eq!(for_terminal(word), Err(NO_NETWORK));
    }
    assert_eq!(NO_NETWORK, "a terminal run has no network in this release");
}

#[test]
fn every_box_without_a_destination_is_as_parsed() {
    for word in [&b"strict"[..], b"offline", b"allow:/tmp/a,/home/b"] {
        assert_eq!(for_terminal(word), parse(word));
    }
    assert_eq!(for_terminal(b"lenient"), parse(b"lenient"));
}
