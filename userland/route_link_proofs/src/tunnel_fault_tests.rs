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

//! Answers that cannot be believed, from an open stream: no marker, a wrong
//! one, a length past the buffer or past any answer a proxy builds, and a
//! far end that sends without being read. Each is an error with a reason,
//! and none of the answer's bytes reach the reader.

use crate::answer::{ANSWER_BUF, ANSWER_MAX};
use crate::bounds::PENDING_MAX;
use crate::fake::FakeProxy;
use crate::refusal::{Proxy, OVERRUN};
use crate::tunnel::Tunnel;

fn opened(fake: FakeProxy, proxy: Proxy) -> Tunnel<FakeProxy> {
    match Tunnel::open(fake, proxy, "example.com", 443) {
        Ok(t) => t,
        Err(why) => panic!("{why}"),
    }
}

fn garbled_by(raw: (i64, Vec<u8>)) -> (Result<usize, &'static str>, usize) {
    let mut fake = FakeProxy::nym();
    fake.raw.push_back(raw);
    let mut t = opened(fake, Proxy::Nym);
    let mut buf = [0u8; 64];
    let got = t.read(&mut buf);
    (got, t.pending.len())
}

#[test]
fn an_answer_with_no_marker_is_refused() {
    assert_eq!(garbled_by((0, vec![])), (Err(Proxy::Nym.garbled()), 0));
}

#[test]
fn an_answer_with_a_marker_no_proxy_sends_is_refused() {
    for m in [3u8, 7, 0x80, 0xFF] {
        assert_eq!(garbled_by((3, vec![m, b'o', b'k'])), (Err(Proxy::Nym.garbled()), 0));
    }
}

/* A proxy restarted under a stream says it lost it, and the reader is told
 * that rather than that the answer could not be read. */
#[test]
fn a_lost_conversation_is_said_to_be_lost() {
    assert_eq!(garbled_by((1, vec![2])), (Err(Proxy::Nym.lost()), 0));
    let mut fake = FakeProxy::nym();
    fake.raw.push_back((1, vec![2]));
    let mut t = opened(fake, Proxy::Anyone);
    let mut buf = [0u8; 8];
    assert_eq!(t.read(&mut buf), Err(Proxy::Anyone.lost()));
    assert!(t.ended(), "nothing more is asked of a stream the proxy lost");
}

#[test]
fn a_length_past_the_buffer_is_refused() {
    let raw = (ANSWER_BUF as i64 + 1, vec![0u8; 16]);
    assert_eq!(garbled_by(raw), (Err(Proxy::Nym.garbled()), 0));
    assert_eq!(garbled_by((i64::MAX, vec![0u8; 16])), (Err(Proxy::Nym.garbled()), 0));
}

#[test]
fn an_answer_longer_than_any_proxy_builds_is_refused() {
    let raw = (ANSWER_MAX as i64 + 1, vec![0u8; ANSWER_MAX + 1]);
    assert_eq!(garbled_by(raw), (Err(Proxy::Nym.garbled()), 0));
    /* The longest real one is read. */
    let mut fake = FakeProxy::nym();
    fake.raw.push_back((ANSWER_MAX as i64, vec![0u8; ANSWER_MAX]));
    let mut t = opened(fake, Proxy::Nym);
    let mut buf = vec![0u8; ANSWER_MAX];
    assert_eq!(t.read(&mut buf), Ok(ANSWER_MAX - 1));
}

#[test]
fn a_short_answer_is_just_fewer_bytes() {
    let mut fake = FakeProxy::nym();
    fake.raw.push_back((2, vec![0, b'h']));
    fake.raw.push_back((1, vec![0]));
    fake.raw.push_back((3, vec![1, b'i', b'!']));
    let mut t = opened(fake, Proxy::Nym);
    let mut buf = [0u8; 8];
    assert_eq!(t.read(&mut buf), Ok(1));
    assert_eq!(buf[0], b'h');
    assert_eq!(t.read(&mut buf), Ok(0));
    assert_eq!(t.read(&mut buf), Ok(2));
    assert_eq!(&buf[..2], b"i!");
    assert!(t.ended());
}

#[test]
fn a_far_end_that_sends_without_being_read_is_cut_off() {
    let mut fake = FakeProxy::nym();
    fake.flood = 32 * 1024;
    let mut t = opened(fake, Proxy::Anyone);
    let big = vec![1u8; 128 * 16 * 1024];
    assert_eq!(t.write_all(&big), Err(OVERRUN));
    assert!(t.pending.len() <= PENDING_MAX);
}
