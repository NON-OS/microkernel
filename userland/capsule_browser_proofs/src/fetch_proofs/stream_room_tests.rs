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

//! The streams a proxy gives the browser. When every one was held, a new
//! fetch failed with "net.socks5 is not running, so nothing was sent", of a
//! proxy that was running; kept connections were held under the direct
//! limit of eight, more than the proxy's streams, so a navigation after a
//! busy page could find none. Now a pool fetch waits its turn for a stream,
//! kept proxied connections stay under the proxied limit, and a navigation
//! frees one before it opens.

use super::fetch_fixtures::url_of;
use super::fetch_wire::FakeWire;
use crate::browser::fetch::open::{open, NO_STREAM};
use crate::browser::fetch::pool::{Idle, Pool, Refused};
use crate::browser::fetch::wire::Wire;
use crate::browser::net::mixnet::{Network, Way};
use nonos_route_link::Proxy;

const NYM: Way = Way::Proxy { net: Network::Nym, port: 41, proxy: Proxy::Nym };

fn kept(w: &mut FakeWire, host: &str) -> Idle {
    let handle = w.open(NYM).expect("a stream");
    Idle {
        host: String::from(host),
        port: 443,
        https: true,
        handle,
        tls: None,
        buf: Vec::new(),
        consumed: 0,
        tx_seq: 1,
        used: 1,
        since_ms: 0,
        way: NYM,
    }
}

#[test]
fn a_fetch_with_no_stream_free_waits_its_turn() {
    let mut w = FakeWire::at(0);
    w.mixnet = true;
    w.streams = Some(2);
    for _ in 0..2 {
        w.open(NYM).expect("held elsewhere, by the navigation and a kept one");
    }
    let mut pool = Pool::new();
    let got = pool.start(&mut w, url_of("https://a.test/x.png"), None).map(|_| ());
    assert_eq!(got, Err(Refused::Busy), "queued, not failed as a proxy not running");
}

#[test]
fn opening_with_no_stream_free_says_so() {
    let mut w = FakeWire::at(0);
    w.mixnet = true;
    w.streams = Some(1);
    w.open(NYM).expect("held");
    let got = open(&mut w, url_of("https://a.test/"), None).map(|_| ());
    assert_eq!(got, Err(NO_STREAM));
    assert_ne!(got, Err(Network::Nym.absent()), "the proxy is running");
}

#[test]
fn kept_proxied_connections_stay_under_the_proxied_limit() {
    let mut w = FakeWire::at(0);
    w.mixnet = true;
    let mut pool = Pool::new();
    for host in ["a.test", "b.test", "c.test", "d.test", "e.test", "f.test"] {
        let idle = kept(&mut w, host);
        pool.park(&mut w, idle);
    }
    let (_, proxied) = Pool::limits(true);
    assert_eq!(pool.idle.len(), proxied, "not the direct limit of eight");
    assert_eq!(w.closed.len(), 6 - proxied, "the oldest were let go");
}

#[test]
fn a_navigation_frees_a_stream_kept_first_then_running() {
    let mut w = FakeWire::at(0);
    w.mixnet = true;
    w.streams = Some(3);
    let mut pool = Pool::new();
    assert!(pool.start(&mut w, url_of("https://a.test/1.png"), None).is_ok());
    assert!(pool.start(&mut w, url_of("https://b.test/2.png"), None).is_ok());
    let idle = kept(&mut w, "c.test");
    let kept_handle = idle.handle;
    pool.park(&mut w, idle);
    assert!(!w.room(NYM));
    assert!(pool.free_stream(&mut w, NYM));
    assert_eq!(w.closed, [kept_handle], "the kept connection goes first");
    assert_eq!(pool.live.len(), 2, "the running fetches stay while that is enough");
    w.open(NYM).expect("the navigation's");
    assert!(pool.free_stream(&mut w, NYM));
    assert_eq!(pool.live.len(), 1, "then the oldest running fetch of the page being left");
    assert!(w.room(NYM));
}
