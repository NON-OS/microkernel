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

//! Through a network's proxy each fetch is a stream of its own, and a
//! finished one is kept for the next request to its host, which then skips
//! the SOCKS handshake and the tunnel: seconds of round trips on a mixnet.

use super::fetch_fixtures::url_of;
use super::fetch_wire::FakeWire;
use super::pool_reuse_tests::OK;
use crate::browser::fetch::pool::{retain, Pool};
use crate::browser::fetch::types::Phase;
use crate::browser::net::mixnet::frames::{numbered, reset};
use crate::browser::net::mixnet::streams::{lowest_free, STREAMS};
use crate::browser::net::mixnet::Network;

#[test]
fn a_request_on_a_kept_anyone_stream_reads_the_anyone_jar() {
    let now = 1_791_028_800;
    let host_url = url_of("http://kept-jar.example/");
    let set = |v: &str| format!("HTTP/1.1 200 OK\r\nSet-Cookie: {v}; Path=/\r\n\r\n");
    crate::browser::cookie::store::absorb(Network::Anyone, &host_url, set("a=1").as_bytes(), now);
    crate::browser::cookie::store::absorb(Network::Nym, &host_url, set("n=1").as_bytes(), now);
    let mut w = FakeWire::at(0);
    w.mixnet = true;
    w.anyone = true;
    w.writable = true;
    let mut pool = Pool::new();
    let handle = pool.start(&mut w, url_of("http://kept-jar.example/a"), None).expect("start").handle;
    pool.step(&mut w, 30);
    w.deliver(handle, &[5, 0]);
    pool.step(&mut w, 30);
    w.deliver(handle, &CONNECTED);
    pool.step(&mut w, 30);
    w.deliver(handle, OK);
    let mut job = pool.step(&mut w, 30).remove(0);
    assert_eq!(job.net(), Network::Anyone);
    let body = core::mem::take(&mut job.buf);
    pool.park(&mut w, retain(&mut job, &body, 0).expect("kept"));
    let before = w.sent.len();
    let next = pool.start(&mut w, url_of("http://kept-jar.example/b"), None).expect("start");
    assert_eq!((next.handle, next.net()), (handle, Network::Anyone));
    let sent = &w.sent[before].1;
    let has = |l: &str| sent.windows(l.len()).any(|x| x == l.as_bytes());
    assert!(has("\r\nCookie: a=1\r\n") && !has("n=1"), "{}", String::from_utf8_lossy(sent));
}

const CONNECTED: [u8; 10] = [5, 0, 0, 1, 0, 0, 0, 0, 0, 0];

#[test]
fn a_new_conversation_names_the_lowest_free_stream_and_no_more_than_the_proxy_gives() {
    assert_eq!(lowest_free(&[]), Some(1));
    assert_eq!(lowest_free(&[1, 2, 4]), Some(3));
    let all: Vec<u32> = (1..=STREAMS).collect();
    assert_eq!(lowest_free(&all), None);
    /* The proxy gives one caller eight, stream 0 among them. */
    const { assert!(STREAMS < 8) };
}

#[test]
fn a_named_stream_goes_in_the_new_frames_and_stream_0_in_the_old() {
    assert_eq!(reset(0), vec![1]);
    assert_eq!(reset(3), vec![3, 3, 0, 0, 0]);
    assert_eq!(numbered(0, 5, b"x"), vec![2, 5, 0, 0, 0, b'x']);
    assert_eq!(numbered(2, 5, b"x"), vec![4, 2, 0, 0, 0, 5, 0, 0, 0, b'x']);
}

#[test]
fn a_proxied_connection_is_kept_and_its_next_request_skips_the_handshake() {
    let mut w = FakeWire::at(0);
    w.mixnet = true;
    w.writable = true;
    let mut pool = Pool::new();
    let handle = pool.start(&mut w, url_of("http://example.org/a.png"), None).expect("start").handle;
    assert!(pool.step(&mut w, 30).is_empty());
    w.deliver(handle, &[5, 0]);
    assert!(pool.step(&mut w, 30).is_empty());
    w.deliver(handle, &CONNECTED);
    assert!(pool.step(&mut w, 30).is_empty(), "the request went");
    w.deliver(handle, OK);
    let mut done = pool.step(&mut w, 30);
    let mut job = done.remove(0);
    assert!(job.way.proxied() && job.keep);
    let body = core::mem::take(&mut job.buf);
    let idle = retain(&mut job, &body, 0).expect("framed, kept");
    pool.park(&mut w, idle);
    let sent_before = w.sent.len();
    let second = pool.start(&mut w, url_of("http://example.org/b.png"), None).expect("start");
    assert_eq!((second.handle, second.phase, second.keep_uses), (handle, Phase::ReadBody, 1));
    assert!(second.way.proxied(), "the kept way goes with it");
    assert_eq!(w.opened.len(), 1, "no second stream");
    assert_eq!(w.sent.len(), sent_before + 1, "only the request: no greeting, no CONNECT");
}

#[test]
fn a_kept_connection_never_carries_a_request_whose_host_now_goes_another_way() {
    let mut w = FakeWire::at(0);
    w.mixnet = true;
    w.writable = true;
    let mut pool = Pool::new();
    let handle = pool.start(&mut w, url_of("http://example.org/a"), None).expect("start").handle;
    pool.step(&mut w, 30);
    w.deliver(handle, &[5, 0]);
    pool.step(&mut w, 30);
    w.deliver(handle, &CONNECTED);
    pool.step(&mut w, 30);
    w.deliver(handle, OK);
    let mut job = pool.step(&mut w, 30).remove(0);
    let body = core::mem::take(&mut job.buf);
    let idle = retain(&mut job, &body, 0).expect("kept");
    pool.park(&mut w, idle);
    w.anyone = true;
    let next = pool.start(&mut w, url_of("http://example.org/b"), None).expect("start");
    assert_ne!(next.handle, handle, "Nym's stream is not used for Anyone");
    assert_eq!(next.phase, Phase::SocksHello, "a fresh stream, its own handshake");
}
