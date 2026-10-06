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

//! A tunnel worked in slices: no call waits longer than its slice, an
//! answer that did not come in a slice is asked for again under the same
//! number and taken once, a network not connected yet is tried again after
//! its pause, and a proxy that never answers is silent within the bound
//! the blocking path keeps.

use crate::bounds::{ASKS, OPEN_MS, REASK_GAP_MS, SEND_WAIT_MS};
use crate::carrier::Carrier;
use crate::fake::{FakeProxy, Shared};
use crate::opening::Opening;
use crate::refusal::Proxy;
use crate::slice::Slice;
use crate::tunnel::Tunnel;

const SLICE: u64 = 50;

/* The time a window's tick leaves between slices. */
const TICK: u64 = 40;

/* A carrier that remembers the longest any call was let wait. */
struct Timed {
    inner: Shared,
    longest: u64,
}

impl Carrier for Timed {
    fn call(&mut self, frame: &[u8], wait_ms: u64, into: &mut [u8]) -> i64 {
        self.longest = self.longest.max(wait_ms);
        self.inner.call(frame, wait_ms, into)
    }

    fn now_ms(&self) -> i64 {
        self.inner.now_ms()
    }

    fn pause(&mut self, ms: u64) {
        self.inner.pause(ms)
    }
}

fn timed(fake: FakeProxy) -> (Timed, Shared) {
    let shared = Shared::new(fake);
    (Timed { inner: shared.clone(), longest: 0 }, shared)
}

/* Step until Done or Failed, a tick between steps; the steps it took. */
fn run<C: Carrier>(o: &mut Opening<C>, fake: &Shared, most: usize) -> (Slice, usize) {
    for n in 1..=most {
        match o.step(SLICE) {
            Slice::Waiting => fake.0.borrow_mut().pause(TICK),
            end => return (end, n),
        }
    }
    (Slice::Waiting, most)
}

fn opened(fake: FakeProxy) -> (Tunnel<Timed>, Shared) {
    let (carrier, shared) = timed(fake);
    let Ok(mut o) = Opening::begin(carrier, Proxy::Nym, "example.com", 443, SLICE) else {
        panic!("did not begin")
    };
    assert_eq!(run(&mut o, &shared, 100).0, Slice::Done);
    (o.into_tunnel(), shared)
}

#[test]
fn nothing_is_sent_before_the_first_step() {
    let (carrier, shared) = timed(FakeProxy::nym());
    let o = Opening::begin(carrier, Proxy::Nym, "example.com", 443, SLICE);
    assert!(o.is_ok());
    assert_eq!(shared.0.borrow().calls, 0);
}

#[test]
fn a_stream_opens_in_slices_as_the_blocking_open_does() {
    let (t, shared) = opened(FakeProxy::nym());
    let fake = shared.0.borrow();
    assert_eq!(fake.frames[0], [1], "a reset comes first");
    let n = fake.numbered();
    assert_eq!(n[0], (1, vec![5, 1, 0]));
    assert_eq!(n[1].0, 2);
    assert_eq!(fake.host, b"example.com");
    assert_eq!(fake.port, 443);
    assert!(t.pending.is_empty() && !t.closed && t.seq == 3);
    assert!(t.carrier.longest <= SLICE, "no call waited past its slice");
}

#[test]
fn net_anon_is_polled_in_slices_until_its_exit_connects() {
    let (carrier, shared) = timed(FakeProxy::anon(6));
    let Ok(mut o) = Opening::begin(carrier, Proxy::Anyone, "example.com", 443, SLICE) else {
        panic!("did not begin")
    };
    assert_eq!(run(&mut o, &shared, 100).0, Slice::Done);
    let polls = shared.0.borrow().numbered().len() - 2;
    assert_eq!(polls, 6);
    assert!(o.into_tunnel().carrier.longest <= SLICE);
}

#[test]
fn a_network_not_connected_yet_is_asked_again_after_its_pause_then_refused() {
    let mut fake = FakeProxy::nym();
    fake.rep = 3;
    let (carrier, shared) = timed(fake);
    let Ok(mut o) = Opening::begin(carrier, Proxy::Nym, "example.com", 443, SLICE) else {
        panic!("did not begin")
    };
    let (end, _) = run(&mut o, &shared, 10_000);
    assert_eq!(end, Slice::Failed(Proxy::Nym.refused(3)));
    let fake = shared.0.borrow();
    assert!(fake.resets > 10, "tried again from a reset each time");
    assert!(fake.now - 1_000 >= OPEN_MS, "only once the open's bound had passed");
}

#[test]
fn a_full_proxy_and_a_refused_connect_end_the_open() {
    let mut full = FakeProxy::nym();
    full.method = 0xFF;
    let (carrier, shared) = timed(full);
    let Ok(mut o) = Opening::begin(carrier, Proxy::Nym, "example.com", 443, SLICE) else {
        panic!("did not begin")
    };
    assert_eq!(run(&mut o, &shared, 100).0, Slice::Failed(Proxy::Nym.full()));

    let mut refused = FakeProxy::nym();
    refused.rep = 4;
    let (carrier, shared) = timed(refused);
    let Ok(mut o) = Opening::begin(carrier, Proxy::Anyone, "example.com", 443, SLICE) else {
        panic!("did not begin")
    };
    assert_eq!(run(&mut o, &shared, 100).0, Slice::Failed(Proxy::Anyone.refused(4)));
}

#[test]
fn a_proxy_that_never_answers_is_silent_within_the_blocking_bound() {
    let mut fake = FakeProxy::nym();
    fake.dead = true;
    let (carrier, shared) = timed(fake);
    let Ok(mut o) = Opening::begin(carrier, Proxy::Nym, "example.com", 443, SLICE) else {
        panic!("did not begin")
    };
    let (end, _) = run(&mut o, &shared, 100_000);
    assert_eq!(end, Slice::Failed(Proxy::Nym.silent()));
    let spent = shared.0.borrow().now - 1_000;
    let bound = (SEND_WAIT_MS * ASKS as u64) as i64;
    assert!(spent >= bound && spent < bound + 1_000, "spent {spent}");
    assert!(o.into_tunnel().carrier.longest <= SLICE);
}

#[test]
fn an_unanswered_exchange_is_not_asked_again_inside_the_gap() {
    let (mut t, shared) = opened(FakeProxy::nym());
    assert_eq!(t.write_slice(b"hello", SLICE), Ok(5));
    shared.0.borrow_mut().lose_next_delivery = true;
    shared
        .0
        .borrow_mut()
        .deliveries
        .push_back(crate::fake::Delivery { after_polls: 0, bytes: b"world".to_vec() });
    let mut buf = [0u8; 16];
    assert_eq!(t.read_slice(&mut buf, SLICE), Ok(0), "the answer was lost");
    let calls = shared.0.borrow().calls;
    assert_eq!(t.read_slice(&mut buf, SLICE), Ok(0));
    assert_eq!(shared.0.borrow().calls, calls, "no call inside the gap");
    shared.0.borrow_mut().pause(REASK_GAP_MS as u64);
    assert_eq!(t.read_slice(&mut buf, SLICE), Ok(5));
    assert_eq!(&buf[..5], b"world");
    let n = shared.0.borrow().numbered();
    let last = &n[n.len() - 2..];
    assert_eq!(last[0].0, last[1].0, "asked again under the same number");
}

#[test]
fn a_write_whose_answer_was_lost_is_carried_once() {
    let (mut t, shared) = opened(FakeProxy::nym());
    shared.0.borrow_mut().lose_next_write = true;
    assert_eq!(t.write_slice(b"hello", SLICE), Ok(0));
    shared.0.borrow_mut().pause(REASK_GAP_MS as u64);
    assert_eq!(t.write_slice(b"hello", SLICE), Ok(5));
    assert_eq!(shared.0.borrow().to_exit, b"hello");
}

#[test]
fn a_sliced_stream_is_reset_as_it_is_dropped_without_waiting_past_its_slice() {
    let (t, shared) = opened(FakeProxy::nym());
    let mut fake = shared.0.borrow_mut();
    fake.dead = true;
    let before = fake.now;
    drop(fake);
    drop(t);
    let fake = shared.0.borrow();
    assert_eq!(fake.frames.last().map(|f| f.as_slice()), Some(&[1u8][..]));
    assert!(fake.now - before <= SLICE as i64 + 1, "the reset waited only its slice");
}

#[test]
fn a_close_is_read_as_the_end() {
    let mut fake = FakeProxy::nym().deliver(2, b"last");
    fake.close_after = true;
    let (mut t, shared) = opened(fake);
    assert_eq!(t.write_slice(b"q", SLICE), Ok(1));
    let mut got = Vec::new();
    let mut buf = [0u8; 8];
    for _ in 0..20 {
        let n = t.read_slice(&mut buf, SLICE).unwrap_or(0);
        got.extend_from_slice(&buf[..n]);
        if t.ended() {
            break;
        }
        shared.0.borrow_mut().pause(TICK);
    }
    assert!(t.ended());
    assert_eq!(got, b"last");
}
