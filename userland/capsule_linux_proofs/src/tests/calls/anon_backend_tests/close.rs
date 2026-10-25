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

//! The last holder closing, and shutdown of both sides.

use super::adopt::{anon_socket, quiet, ANON, GUEST, OTHER};
use crate::linux::net::anon_stream;
use crate::linux::net::sock::{Close, Socks};

#[test]
fn the_last_holder_closing_closes_the_stream_once() {
    quiet();
    let mut t = Socks::new();
    let id = anon_socket(&mut t, 4);
    t.get_mut(id).expect("there").holders.push(OTHER);
    t.release(id, GUEST);
    assert!(anon_stream::closed().is_empty(), "another process still holds it");
    t.release(id, OTHER);
    assert_eq!(anon_stream::closed(), vec![Close { port: ANON, id: 4 }]);
    t.release(id, OTHER);
    t.release(id, GUEST);
    assert!(anon_stream::closed().is_empty());
}

#[test]
fn shutdown_of_both_sides_closes_it_and_freeing_after_sends_nothing() {
    quiet();
    let mut t = Socks::new();
    let id = anon_socket(&mut t, 5);
    let s = t.get_mut(id).expect("there");
    assert_eq!(s.shut_anon(true, true), Ok(()));
    assert!(s.rd_shut && s.wr_shut);
    assert_eq!(anon_stream::closed(), vec![Close { port: ANON, id: 5 }]);
    assert_eq!(t.get_mut(id).expect("still there").shut_anon(true, true), Ok(()));
    assert!(anon_stream::closed().is_empty(), "a second shutdown sends nothing");
    t.release(id, GUEST);
    assert!(t.get(id).is_none(), "the last close lets the entry go");
    assert!(anon_stream::closed().is_empty(), "nor does the close after it");
}
