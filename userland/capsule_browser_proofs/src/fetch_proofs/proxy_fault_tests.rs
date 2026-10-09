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

//! A proxy that was restarted under a page, left its port, or answered with
//! bytes that are not an answer is named for what it did. Each of these
//! used to read as a closed stream, and a closed stream as the exit hanging
//! up; a page with no length was even taken as whole at such a close.

use super::fetch_fixtures::{step, url_of};
use super::fetch_wire::FakeWire;
use crate::browser::fetch::closed::EXIT_CLOSED;
use crate::browser::fetch::proxy_fault::{code, PROXY_GARBLED, PROXY_GONE, PROXY_LOST};
use crate::browser::fetch::retryable_error::{retry_nav, retryable_error};
use crate::browser::fetch::types::{Fetch, Phase};
use crate::browser::net::mixnet::conv::Conv;
use crate::browser::net::mixnet::frames::{ANSWER_MAX, STREAM_LOST};
use crate::browser::net::mixnet::Broke;

const H: u32 = 11;

fn opened() -> Conv {
    let mut c = Conv::opening(2);
    let _ = c.frame();
    assert!(c.answered(&[0]), "the reset");
    c
}

#[test]
fn a_proxy_that_lost_the_conversation_says_so() {
    let mut c = opened();
    c.take(b"GET").expect("taken");
    let _ = c.frame();
    assert!(!c.answered(&[STREAM_LOST]));
    assert!(c.ended(), "nothing more is asked");
    assert_eq!(c.why_broken(), Some(Broke::Lost));
}

#[test]
fn an_answer_longer_than_any_proxy_builds_is_refused() {
    let mut c = opened();
    let _ = c.frame();
    let over = vec![0u8; ANSWER_MAX + 1];
    assert!(!c.answered(&over));
    assert_eq!(c.why_broken(), Some(Broke::Garbled));
    let mut c = opened();
    let _ = c.frame();
    assert!(c.answered(&vec![0u8; ANSWER_MAX]), "the longest real answer is read");
    assert_eq!(c.why_broken(), None);
}

#[test]
fn an_unknown_marker_and_a_refused_call_are_each_named() {
    let mut c = opened();
    let _ = c.frame();
    assert!(!c.answered(&[9, 1, 2]));
    assert_eq!(c.why_broken(), Some(Broke::Garbled));
    let mut c = opened();
    c.refused();
    assert_eq!(c.why_broken(), Some(Broke::Gone));
    let mut c = opened();
    let _ = c.frame();
    assert!(c.answered(&[1]), "a close is the far end's");
    assert!(c.ended());
    assert_eq!(c.why_broken(), None, "and is not the proxy breaking");
}

#[test]
fn each_way_a_proxy_breaks_has_its_own_code() {
    assert_eq!(code(Broke::Gone), PROXY_GONE);
    assert_eq!(code(Broke::Garbled), PROXY_GARBLED);
    assert_eq!(code(Broke::Lost), PROXY_LOST);
}

#[test]
fn a_page_cut_by_a_restarted_proxy_is_not_whole() {
    let mut w = FakeWire::at(0);
    w.mixnet = true;
    let mut f = Fetch::new(url_of("http://example.org/"), H, Phase::ReadBody, 0);
    w.deliver(H, b"HTTP/1.1 200 OK\r\n\r\n<p>half");
    w.finished.push(H);
    w.broken.push((H, PROXY_LOST));
    step(&mut w, &mut f);
    assert_eq!(f.error, Some(PROXY_LOST), "a page with no length is not whole at a lost proxy");
}

#[test]
fn a_close_the_proxy_caused_is_not_the_exit_s() {
    let mut w = FakeWire::at(0);
    w.mixnet = true;
    let mut f = crate::browser::fetch::open::open(&mut w, url_of("https://example.org/"), None)
        .expect("open");
    step(&mut w, &mut f);
    w.deliver(f.handle, &[5, 0]);
    step(&mut w, &mut f);
    w.deliver(f.handle, &[5, 0, 0, 1, 0, 0, 0, 0, 0, 0]);
    step(&mut w, &mut f);
    assert_eq!(f.phase, Phase::TlsFlight);
    w.finished.push(f.handle);
    w.broken.push((f.handle, PROXY_GONE));
    step(&mut w, &mut f);
    assert_eq!(f.error, Some(PROXY_GONE));
    assert_ne!(f.error, Some(EXIT_CLOSED));
}

#[test]
fn a_proxy_that_went_away_or_restarted_is_asked_again_from_the_start() {
    assert!(retryable_error(PROXY_GONE) && retryable_error(PROXY_LOST));
    assert!(!retryable_error(PROXY_GARBLED), "the same proxy would answer the same");
    assert!(!retry_nav(PROXY_LOST, true, true), "a POST that may have been acted on is not sent twice");
}
