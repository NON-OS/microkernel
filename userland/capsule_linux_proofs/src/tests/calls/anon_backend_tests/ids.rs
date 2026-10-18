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

//! An id already held, a socket gone, and the stream's own bookkeeping.

use super::adopt::{anon_socket, quiet, ANON, GUEST, OTHER, TO};
use crate::linux::abi::errno::{EBADF, EIO};
use crate::linux::net::anon_stream;
use crate::linux::net::sock::{Anon, Close, Domain, Proto, Socks};

#[test]
fn an_id_a_socket_here_already_holds_is_refused_and_left_open() {
    quiet();
    let mut t = Socks::new();
    let first = anon_socket(&mut t, 11);
    let second = t.open(Domain::Inet, Proto::Stream, Some(OTHER));
    assert_eq!(t.adopt_anon(second, ANON, 11, TO), Err(EIO));
    let s = t.get(second).expect("there");
    assert!(s.svc.is_none() && !s.connected, "the second socket stays unconnected");
    assert!(anon_stream::closed().is_empty(), "closing it would end the first socket's stream");
    assert!(t.holds_anon(ANON, 11));
    t.free(second, false);
    assert!(anon_stream::closed().is_empty());
    t.free(first, false);
    assert_eq!(anon_stream::closed(), vec![Close { port: ANON, id: 11 }]);
    assert!(!t.holds_anon(ANON, 11));
}

#[test]
fn a_stream_opened_for_a_socket_that_went_is_closed_again() {
    quiet();
    let mut t = Socks::new();
    let id = t.open(Domain::Inet, Proto::Stream, Some(GUEST));
    t.free(id, false);
    assert_eq!(t.adopt_anon(id, ANON, 12, TO), Err(EBADF));
    assert_eq!(anon_stream::closed(), vec![Close { port: ANON, id: 12 }]);
}

#[test]
fn a_stream_is_closed_once_by_its_own_bookkeeping() {
    let mut a = Anon::new(ANON, 13);
    assert_eq!(a.close(), Some(Close { port: ANON, id: 13 }));
    assert_eq!(a.close(), None);
    assert!(!a.wants_fill(), "nothing is read from a stream once closed");
}
