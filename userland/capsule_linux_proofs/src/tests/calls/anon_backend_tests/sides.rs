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

//! One side shut alone: the reading side waits for the socket, the writing side is refused.

use super::adopt::{anon_socket, quiet, ANON};
use crate::linux::abi::errno::EOPNOTSUPP;
use crate::linux::net::anon_stream;
use crate::linux::net::sock::{Close, Socks};

#[test]
fn shutting_the_reading_side_closes_nothing_until_the_socket_goes() {
    quiet();
    let mut t = Socks::new();
    let id = anon_socket(&mut t, 6);
    let s = t.get_mut(id).expect("there");
    assert_eq!(s.shut_anon(true, false), Ok(()));
    assert!(s.rd_shut && !s.wr_shut);
    assert!(anon_stream::closed().is_empty());
    t.free(id, false);
    assert_eq!(anon_stream::closed(), vec![Close { port: ANON, id: 6 }]);
}

#[test]
fn shutting_the_writing_side_alone_is_refused_and_changes_nothing() {
    quiet();
    let mut t = Socks::new();
    let id = anon_socket(&mut t, 7);
    let s = t.get_mut(id).expect("there");
    assert_eq!(s.shut_anon(false, true), Err(EOPNOTSUPP));
    assert!(!s.rd_shut && !s.wr_shut);
    assert!(anon_stream::closed().is_empty());
    t.free(id, false);
    assert_eq!(anon_stream::closed(), vec![Close { port: ANON, id: 7 }]);
}
