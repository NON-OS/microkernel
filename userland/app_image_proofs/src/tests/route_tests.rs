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

//! The fast route is never taken by default: only a build that carries it,
//! with the person's word and a mirror address, takes it. The digests a
//! chunk is held to do not depend on the route (run.rs holds every chunk to
//! ChunkList::holds whichever way it came).

use crate::install::image::route_pick::{asked, route, width, Route};

#[test]
fn the_default_is_the_mixnet() {
    // A release build: no feature, no setting, no fast mirror.
    assert_eq!(route(false, asked(None), None), Route::Mixnet);
    // The capsule's own default build passes cfg!(feature = ...) as false.
    for a in [false, true] {
        for m in [None, Some(""), Some("192.0.2.1:80")] {
            assert_eq!(route(false, a, m), Route::Mixnet, "not built: {a} {m:?}");
        }
    }
    assert_eq!(route(true, false, Some("192.0.2.1:80")), Route::Mixnet, "built, not asked");
    assert_eq!(route(true, true, None), Route::Mixnet, "built and asked, no mirror");
    assert_eq!(route(true, true, Some("")), Route::Mixnet);
    assert_eq!(route(true, true, Some("192.0.2.1:80")), Route::Fast);
}

#[test]
fn only_the_word_on_asks() {
    assert!(!asked(None));
    for s in [&b""[..], b"off", b"On", b"yes", b"1", b"onn", b" o n"] {
        assert!(!asked(Some(s)), "{s:?}");
    }
    for s in [&b"on"[..], b"on\n", b"  on  \n"] {
        assert!(asked(Some(s)), "{s:?}");
    }
}

#[test]
fn either_route_holds_four_chunks_in_flight_and_no_more() {
    assert_eq!(width(Route::Mixnet), 4);
    assert_eq!(width(Route::Fast), 4);
}
