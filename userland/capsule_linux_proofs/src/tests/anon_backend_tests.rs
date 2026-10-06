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

//! A net.anon stream is closed exactly once: on the last close of the
//! socket, on shutdown of both sides, or when its entry is freed, whichever
//! comes first, and never again after. A mixnet stream is let go as it
//! always was. Driven through the socket table as it ships, with the two
//! close calls recorded instead of sent.

use crate::linux::abi::errno::{EBADF, EIO, ENOBUFS, EOPNOTSUPP};
use crate::linux::net::anon_answer::opened;
use crate::linux::net::anon_ops::{E_OK, E_TABLE_FULL};
use crate::linux::net::sock::{Addr, Anon, Backend, Close, Domain, Proto, Socks, Via};
use crate::linux::net::{anon_stream, stream};

const LIMITS: &str = include_str!("../../../capsule_net_anon/src/protocol/limits.rs");
const OWNED: &str = include_str!("../../../capsule_net_anon/src/stream/owned.rs");

const ANON: u32 = 4484;
const GUEST: u32 = 7;
const OTHER: u32 = 8;
const TO: Addr = Addr { ip: [93, 184, 216, 34], port: 443 };

/// A connected stream socket held by GUEST, its stream `sid` on net.anon.
fn anon_socket(t: &mut Socks, sid: u16) -> u32 {
    let id = t.open(Domain::Inet, Proto::Stream, Some(GUEST));
    assert_eq!(t.adopt_anon(id, ANON, sid, TO), Ok(()));
    id
}

fn quiet() {
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

#[test]
fn every_order_of_shutdown_close_and_free_closes_once() {
    for steps in 0u32..(1 << 6) {
        quiet();
        let mut t = Socks::new();
        let id = anon_socket(&mut t, 9);
        for k in 0..3 {
            match (steps >> (2 * k)) & 3 {
                0 => {
                    let _ = t.get_mut(id).map(|s| s.shut_anon(true, true));
                }
                1 => {
                    let _ = t.get_mut(id).map(|s| s.shut_anon(true, false));
                }
                2 => t.release(id, GUEST),
                _ => t.free(id, false),
            }
        }
        t.free(id, false);
        assert_eq!(anon_stream::closed(), vec![Close { port: ANON, id: 9 }], "steps {steps:#b}");
    }
}

#[test]
fn a_mixnet_stream_is_let_go_as_before() {
    quiet();
    let mut t = Socks::new();
    let id = t.open(Domain::Inet, Proto::Stream, Some(GUEST));
    t.get_mut(id).expect("there").svc = Some(Backend::Sockets(21));
    assert_eq!(t.get(id).and_then(|s| s.svc.as_ref()).map(Backend::via), Some(Via::Sockets(21)));
    t.release(id, GUEST);
    assert_eq!(stream::closed(), vec![21]);
    assert!(anon_stream::closed().is_empty(), "net.anon is not told of a mixnet stream");
    t.free(id, false);
    assert!(stream::closed().is_empty());
}

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

#[test]
fn past_net_anons_share_for_one_caller_a_connect_is_enobufs() {
    /* net.anon gives one caller half its table: this capsule, for every guest. */
    assert!(LIMITS.contains("pub const STREAM_MAX: usize = 32;"));
    assert!(OWNED.contains("streams.iter().filter(|s| s.owner == owner).count() < max / 2"));
    let share = 32 / 2;
    quiet();
    let mut t = Socks::new();
    let mut held = 0usize;
    for n in 0..=share {
        let id = t.open(Domain::Inet, Proto::Stream, Some(GUEST));
        /* What net.anon answers this caller's open, as may_open decides it. */
        let sid = (n as u16 + 1).to_le_bytes();
        let answer = if held < share { (E_OK, &sid[..]) } else { (E_TABLE_FULL, &[][..]) };
        match opened(Some(answer)) {
            Ok(sid) => {
                assert_eq!(t.adopt_anon(id, ANON, sid, TO), Ok(()));
                held += 1;
            }
            Err(e) => {
                assert_eq!((n, e), (share, ENOBUFS), "the stream past the share, and only it");
                let s = t.get(id).expect("there");
                assert!(s.svc.is_none() && !s.connected, "nothing was adopted");
            }
        }
    }
    assert_eq!(held, share);
    assert!(anon_stream::closed().is_empty(), "no stream is closed by a refusal");
}
