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

//! A stream adopted by its socket, and freed once.

use crate::linux::net::sock::{Addr, Backend, Close, Domain, Proto, Socks, Via};
use crate::linux::net::{anon_stream, stream};

pub(super) const LIMITS: &str =
    include_str!("../../../../../capsule_net_anon/src/protocol/limits.rs");
pub(super) const OWNED: &str = include_str!("../../../../../capsule_net_anon/src/stream/owned.rs");
pub(super) const ANON: u32 = 4484;
pub(super) const GUEST: u32 = 7;
pub(super) const OTHER: u32 = 8;
pub(super) const TO: Addr = Addr { ip: [93, 184, 216, 34], port: 443 };

/// A connected stream socket held by GUEST, its stream `sid` on net.anon.
pub(super) fn anon_socket(t: &mut Socks, sid: u16) -> u32 {
    let id = t.open(Domain::Inet, Proto::Stream, Some(GUEST));
    assert_eq!(t.adopt_anon(id, ANON, sid, TO), Ok(()));
    id
}

pub(super) fn quiet() {
    let _ = anon_stream::closed();
    let _ = stream::closed();
}

#[test]
fn adopting_a_stream_connects_the_socket_to_it() {
    quiet();
    let mut t = Socks::new();
    let id = anon_socket(&mut t, 3);
    let s = t.get(id).expect("the socket is there");
    assert!(s.connected);
    assert!(s.remote == Some(TO));
    assert_eq!(s.svc.as_ref().map(Backend::via), Some(Via::Anon));
    let a = s.svc.as_ref().and_then(Backend::anon).expect("a net.anon stream");
    assert_eq!((a.port, a.id), (ANON, Some(3)));
    assert!(anon_stream::closed().is_empty(), "nothing is closed on the way in");
}

#[test]
fn freeing_the_entry_closes_the_stream_once() {
    quiet();
    let mut t = Socks::new();
    let id = anon_socket(&mut t, 3);
    t.free(id, false);
    assert_eq!(anon_stream::closed(), vec![Close { port: ANON, id: 3 }]);
    t.free(id, false);
    assert!(anon_stream::closed().is_empty(), "a freed entry is not freed again");
    assert!(stream::closed().is_empty(), "net.sockets is not told of a net.anon stream");
}
