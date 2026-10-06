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

//! A close the proxy reports ends the fetch in the step that sees it: as a
//! failure when nothing came, as a whole page when the page had no length,
//! and as a cut one when it stopped short of its framing.

use super::fetch_fixtures::{step, url_of};
use super::fetch_wire::FakeWire;
use crate::browser::fetch::budget::budget;
use crate::browser::fetch::closed::{at_close, AtClose, EXIT_CLOSED};
use crate::browser::fetch::open::open;
use crate::browser::fetch::proxy_fault::PROXY_ENDED;
use crate::browser::fetch::retryable_error::{retry_nav, retryable_error};
use crate::browser::fetch::types::{Fetch, Phase};
use crate::browser::net::drain::drain;
use crate::browser::net::mixnet::Network;

const H: u32 = 11;

fn reading(w: &mut FakeWire) -> Fetch {
    w.mixnet = true;
    Fetch::new(url_of("http://example.org/"), H, Phase::ReadBody, 0)
}

#[test]
fn a_close_before_any_answer_fails_at_once_and_is_not_tried_again() {
    let mut w = FakeWire::at(0);
    w.mixnet = true;
    let mut f = open(&mut w, url_of("https://example.org/"), None).expect("open");
    step(&mut w, &mut f);
    assert_eq!(f.phase, Phase::SocksMethod);
    w.finished.push(f.handle);
    w.advance(30);
    step(&mut w, &mut f);
    assert_eq!(f.error, Some(PROXY_ENDED), "the greeting is the proxy's to answer, not the exit's");
    assert_eq!(EXIT_CLOSED, "the exit closed the connection");
    assert!(!retryable_error(EXIT_CLOSED));
    assert!(!retry_nav(EXIT_CLOSED, false, false));
}

#[test]
fn a_refusal_and_its_close_say_the_refusal() {
    let mut w = FakeWire::at(0);
    w.mixnet = true;
    let mut f = open(&mut w, url_of("https://example.org/"), None).expect("open");
    step(&mut w, &mut f);
    w.deliver(f.handle, &[5, 0]);
    step(&mut w, &mut f);
    assert_eq!(f.phase, Phase::SocksConnect);
    w.deliver(f.handle, &[5, 4, 0, 1, 0, 0, 0, 0, 0, 0]);
    w.finished.push(f.handle);
    step(&mut w, &mut f);
    assert_eq!(f.error, Some(Network::Nym.refused(4)), "the reply is read first");
}

#[test]
fn a_cut_socks_reply_is_a_close() {
    let mut w = FakeWire::at(0);
    w.mixnet = true;
    let mut f = open(&mut w, url_of("https://example.org/"), None).expect("open");
    step(&mut w, &mut f);
    w.deliver(f.handle, &[5, 0]);
    step(&mut w, &mut f);
    w.deliver(f.handle, &[5, 0, 0, 1, 0]);
    w.finished.push(f.handle);
    step(&mut w, &mut f);
    assert_eq!(f.error, Some(PROXY_ENDED), "a proxy sends its reply before it closes");
}

#[test]
fn a_close_before_the_server_flight_fails_as_a_close() {
    let mut w = FakeWire::at(0);
    w.mixnet = true;
    let mut f = open(&mut w, url_of("https://example.org/"), None).expect("open");
    step(&mut w, &mut f);
    w.deliver(f.handle, &[5, 0]);
    step(&mut w, &mut f);
    w.deliver(f.handle, &[5, 0, 0, 1, 0, 0, 0, 0, 0, 0]);
    step(&mut w, &mut f);
    assert_eq!(f.phase, Phase::TlsFlight);
    w.finished.push(f.handle);
    step(&mut w, &mut f);
    assert_eq!(f.error, Some(EXIT_CLOSED));
}

#[test]
fn a_page_with_no_length_is_whole_at_the_close() {
    let mut w = FakeWire::at(0);
    let mut f = reading(&mut w);
    w.deliver(H, b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<p>hi</p>");
    step(&mut w, &mut f);
    assert_eq!(f.phase, Phase::ReadBody, "no length: it could go on");
    w.finished.push(H);
    step(&mut w, &mut f);
    assert_eq!((f.phase, f.truncated), (Phase::Done, false), "whole, without the idle wait");
}

#[test]
fn a_page_cut_short_of_its_length_is_truncated() {
    let mut w = FakeWire::at(0);
    let mut f = reading(&mut w);
    w.deliver(H, b"HTTP/1.1 200 OK\r\nContent-Length: 100\r\n\r\nshort");
    w.finished.push(H);
    step(&mut w, &mut f);
    assert_eq!((f.phase, f.truncated), (Phase::Done, true));
}

#[test]
fn a_chunked_page_cut_before_its_last_chunk_is_truncated() {
    let mut w = FakeWire::at(0);
    let mut f = reading(&mut w);
    w.deliver(H, b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n5\r\nhello\r\n");
    w.finished.push(H);
    step(&mut w, &mut f);
    assert!(f.truncated);
}

#[test]
fn a_close_with_nothing_of_the_response_is_a_close() {
    let mut w = FakeWire::at(0);
    let mut f = reading(&mut w);
    w.finished.push(H);
    step(&mut w, &mut f);
    assert_eq!(f.error, Some(EXIT_CLOSED));
}

#[test]
fn a_kept_connection_closed_under_its_request_is_tried_fresh() {
    let mut w = FakeWire::at(0);
    let mut f = reading(&mut w);
    f.keep_uses = 2;
    w.finished.push(H);
    step(&mut w, &mut f);
    assert_eq!(f.error, Some("kept connection dead"), "land::respond::dead_kept relaunches it");
}

#[test]
fn what_a_close_leaves_is_read_from_the_framing() {
    assert_eq!(at_close(b""), AtClose::Nothing);
    assert_eq!(at_close(b"HTTP/1.1 200 OK\r\n\r\nbody"), AtClose::Whole);
    assert_eq!(at_close(b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\n\r\nbody"), AtClose::Whole);
    assert_eq!(at_close(b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nbody"), AtClose::Truncated);
    assert_eq!(at_close(b"HTTP/1.1 204 No Content\r\n\r\n"), AtClose::Whole);
    assert_eq!(at_close(b"HTTP/1.1 200 OK\r\nConte"), AtClose::Truncated, "no whole head");
}

#[test]
fn a_drain_stops_at_a_close_and_keeps_what_came_before_it() {
    let mut w = FakeWire::at(0);
    w.deliver(H, b"last bytes");
    w.finished.push(H);
    let mut into = Vec::new();
    let read = drain(&mut w, H, &mut into, 1 << 20, 25);
    assert_eq!((read.got, read.closed, into.as_slice()), (10, true, &b"last bytes"[..]));
}

#[test]
fn a_name_with_no_address_is_not_asked_again() {
    assert!(!retryable_error("dns failed"), "the resolver answered; it would answer the same");
    assert!(retryable_error("timed out") && retryable_error("connect failed"));
}

#[test]
fn a_post_is_tried_again_only_if_its_request_never_left() {
    assert!(retry_nav("connect failed", true, false), "nothing reached the server");
    assert!(!retry_nav("timed out", true, true), "the server may have acted on it");
    assert!(retry_nav("timed out", false, true), "a GET may be asked again");
}

#[test]
fn anyone_waits_thirty_seconds_and_nym_keeps_its_three_minutes() {
    assert_eq!(budget(Network::Anyone).silent_ms, 30_000);
    assert_eq!(budget(Network::Nym).silent_ms, 180_000);
    let mut w = FakeWire::at(0);
    w.mixnet = true;
    w.anyone = true;
    let mut f = open(&mut w, url_of("https://example.org/"), None).expect("open");
    step(&mut w, &mut f);
    w.advance(30_001);
    step(&mut w, &mut f);
    assert_eq!(f.error, Some("timed out"), "Anyone: a silent half minute is enough");
    w.anyone = false;
    let mut f = open(&mut w, url_of("https://example.org/"), None).expect("open");
    step(&mut w, &mut f);
    w.advance(30_001);
    step(&mut w, &mut f);
    assert_eq!(f.error, None, "Nym still waits");
}
