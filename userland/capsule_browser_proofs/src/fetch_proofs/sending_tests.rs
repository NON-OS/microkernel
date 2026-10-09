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

//! A fetch whose bytes a proxy has not answered waits for that answer, a
//! step at a time, and never runs ahead of it.

use super::fetch_fixtures::{step, url_of};
use super::fetch_wire::FakeWire;
use crate::browser::fetch::open::open;
use crate::browser::fetch::types::Phase;
use crate::browser::net::mixnet::Network;

fn proxied() -> (FakeWire, crate::browser::fetch::types::Fetch) {
    let mut w = FakeWire::at(0);
    w.mixnet = true;
    let f = open(&mut w, url_of("https://example.org/"), None).expect("open");
    (w, f)
}

#[test]
fn the_greeting_waits_for_the_reset_to_be_answered() {
    let (mut w, mut f) = proxied();
    w.unanswered.push(f.handle);
    for _ in 0..5 {
        w.advance(30);
        step(&mut w, &mut f);
        assert_eq!(f.phase, Phase::SocksHello);
    }
    assert!(w.sent.is_empty(), "nothing goes behind an unanswered frame");
    assert_eq!(w.asks, 5, "asked again once a step, no more");
    w.unanswered.clear();
    step(&mut w, &mut f);
    assert_eq!(f.phase, Phase::SocksMethod);
    assert_eq!(w.sent_on(f.handle), vec![5, 1, 0]);
}

#[test]
fn the_method_reply_is_not_read_before_the_greeting_is_answered() {
    let (mut w, mut f) = proxied();
    step(&mut w, &mut f);
    assert_eq!(f.phase, Phase::SocksMethod);
    w.unanswered.push(f.handle);
    w.deliver(f.handle, &[5, 0]);
    step(&mut w, &mut f);
    assert_eq!((f.phase, f.socks.len()), (Phase::SocksMethod, 0), "the greeting is still sending");
    assert_eq!(w.sent.len(), 1, "no CONNECT yet");
    w.unanswered.clear();
    step(&mut w, &mut f);
    assert_eq!(f.phase, Phase::SocksConnect, "answered: the reply is read and the CONNECT goes");
    assert_eq!(w.sent.len(), 2);
}

#[test]
fn a_send_never_answered_ends_at_the_deadline_and_not_before() {
    let (mut w, mut f) = proxied();
    w.unanswered.push(f.handle);
    let silent = crate::browser::fetch::budget::budget(Network::Nym).silent_ms;
    w.advance(silent);
    step(&mut w, &mut f);
    assert_eq!(f.phase, Phase::SocksHello, "still within the budget");
    w.advance(1);
    step(&mut w, &mut f);
    assert_eq!(f.error, Some("timed out"));
}

#[test]
fn a_proxy_that_refuses_the_call_fails_the_fetch_at_once() {
    let (mut w, mut f) = proxied();
    w.unanswered.push(f.handle);
    w.refusing = true;
    step(&mut w, &mut f);
    assert_eq!(f.error, Some("send failed"));
}

#[test]
fn a_direct_socket_is_never_sending() {
    let mut w = FakeWire::at(0);
    let mut f = open(&mut w, url_of("http://10.0.2.2/"), None).expect("open");
    w.writable = true;
    step(&mut w, &mut f);
    assert_eq!(f.phase, Phase::ReadBody);
    assert_eq!(w.asks, 0);
}
