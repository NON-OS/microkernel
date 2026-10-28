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

//! Every order of shutdown, close and free, and a stream the routed table holds.

use super::adopt::{anon_socket, quiet, ANON, GUEST};
use crate::linux::net::sock::{Backend, Close, Domain, Proto, Socks, Via};
use crate::linux::net::{anon_stream, routed};

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
fn a_routed_stream_is_let_go_once_and_only_there() {
    quiet();
    let mut t = Socks::new();
    let id = t.open(Domain::Inet, Proto::Stream, Some(GUEST));
    t.get_mut(id).expect("there").svc = Some(Backend::Routed(21));
    assert_eq!(t.get(id).and_then(|s| s.svc.as_ref()).map(Backend::via), Some(Via::Routed(21)));
    t.release(id, GUEST);
    assert_eq!(routed::closed(), vec![(21, false)], "carried on until what was written goes");
    assert!(anon_stream::closed().is_empty(), "net.anon is not told of a routed stream");
    t.free(id, false);
    assert!(routed::closed().is_empty());
}

#[test]
fn an_abortive_close_drops_what_a_routed_stream_has_queued() {
    quiet();
    let mut t = Socks::new();
    let id = t.open(Domain::Inet, Proto::Stream, Some(GUEST));
    let s = t.get_mut(id).expect("there");
    (s.svc, s.opts.linger) = (Some(Backend::Routed(4)), (1, 0));
    t.release(id, GUEST);
    assert_eq!(routed::closed(), vec![(4, true)], "SO_LINGER of 0 s is Linux's reset");
}
