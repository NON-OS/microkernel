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

//! Proofs that one answer to a poll carries everything that has come back
//! for its stream, in order and up to its cap, rather than one message of
//! it; that a 100 KB server flight the mixnet reordered comes back whole in
//! a handful of polls; that a gap holds back only what is behind it; that an
//! answer waits on the network only when it has nothing to give; and that no
//! amount of traffic for other streams keeps a poll from being answered.

use std::collections::VecDeque;

use crate::gather::{gather, Mixnet, Received, PULLS_MAX};
use crate::inbox::Inbox;
use crate::reply::ANSWER_MAX;
use crate::tunnel::{decode_response, ENVELOPE_BYTES, PROTOCOL_VERSION, RESP_NETWORK_DATA};

/// How long an answer waits when it has nothing, as relay.rs sets it.
const HOLD_MS: i64 = 40;

/// What one batched read of net.nym carries at most.
const BATCH_MAX: usize = 32 * 1024;

/// An exit's NetworkData response, in the provider envelope.
fn response(conn: u64, seq: u64, closed: bool, data: &[u8]) -> Vec<u8> {
    let mut out = vec![3, 1, PROTOCOL_VERSION, RESP_NETWORK_DATA, closed as u8];
    assert_eq!(ENVELOPE_BYTES, 2);
    out.extend_from_slice(&conn.to_be_bytes());
    out.extend_from_slice(&seq.to_be_bytes());
    out.extend_from_slice(data);
    out
}

/// What relay.rs does with each message read: decode it and hold it.
fn file(inbox: &mut Inbox, msg: &[u8]) {
    if let Some(r) = decode_response(msg) {
        let _ = inbox.accept(r.conn_id, r.seq, r.closed, r.data);
    }
}

/// net.nym as the proxy sees it: messages land at given times and each read
/// takes as many as fit in one answer. Waiting on an empty link moves the
/// clock on by the wait, or up to the next landing if that is sooner.
struct Net {
    now: i64,
    landing: VecDeque<(i64, Vec<u8>)>,
    queued: VecDeque<Vec<u8>>,
    reads: usize,
    waits: Vec<u32>,
}

impl Net {
    fn new(mut landing: Vec<(i64, Vec<u8>)>) -> Self {
        landing.sort_by_key(|(at, _)| *at);
        Self {
            now: 0,
            landing: landing.into(),
            queued: VecDeque::new(),
            reads: 0,
            waits: Vec::new(),
        }
    }

    fn land(&mut self) {
        while self.landing.front().is_some_and(|(at, _)| *at <= self.now) {
            if let Some((_, m)) = self.landing.pop_front() {
                self.queued.push_back(m);
            }
        }
    }
}

impl Mixnet for Net {
    fn receive(&mut self, wait_ms: u32) -> Received {
        self.reads += 1;
        self.waits.push(wait_ms);
        self.land();
        if self.queued.is_empty() {
            let until = self.now + wait_ms as i64;
            let next = self.landing.front().map(|(at, _)| *at).unwrap_or(i64::MAX);
            self.now = until.min(next.max(self.now));
            self.land();
        }
        if self.queued.is_empty() {
            return Received::Empty;
        }
        let mut out = Vec::new();
        let mut used = 0;
        while let Some(m) = self.queued.front() {
            if !out.is_empty() && used + m.len() > BATCH_MAX {
                break;
            }
            used += m.len();
            out.extend(self.queued.pop_front());
        }
        Received::Messages(out)
    }

    fn now_ms(&self) -> i64 {
        self.now
    }
}

fn pattern(len: usize) -> Vec<u8> {
    (0..len).map(|i| (i * 7 + i / 251) as u8).collect()
}

/// A deterministic shuffle, so a failure replays.
fn shuffle<T>(items: &mut [T], mut seed: u64) {
    for i in (1..items.len()).rev() {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        items.swap(i, ((seed >> 33) % (i as u64 + 1)) as usize);
    }
}

/// Poll the way the browser does until the stream closes or `polls` run out,
/// returning the stream and how many polls it took.
fn read_all(inbox: &mut Inbox, net: &mut Net, conn: u64, polls: usize) -> (Vec<u8>, usize, bool) {
    let mut stream = Vec::new();
    for poll in 1..=polls {
        let got = gather(inbox, conn, ANSWER_MAX, HOLD_MS, net, file);
        stream.extend_from_slice(&got.bytes);
        if got.closed {
            return (stream, poll, true);
        }
        // The browser asks again on its next tick.
        net.now += 16;
    }
    (stream, polls, false)
}

/// A 100 KB server flight sent as 64 messages of 1600 bytes, reordered by
/// the mixnet, already waiting when the browser polls. One message per poll
/// took 64 polls, each a round trip; filling the answer takes four.
#[test]
fn a_hundred_kilobyte_flight_comes_back_whole_in_a_handful_of_polls() {
    let flight = pattern(64 * 1600);
    let mut messages: Vec<Vec<u8>> = flight
        .chunks(1600)
        .enumerate()
        .map(|(seq, chunk)| response(1, seq as u64, seq == 63, chunk))
        .collect();
    shuffle(&mut messages, 0xbeef);
    let mut net = Net::new(messages.into_iter().map(|m| (0, m)).collect());
    let mut inbox = Inbox::default();
    let (stream, polls, closed) = read_all(&mut inbox, &mut net, 1, 200);
    assert!(closed, "the close came with the last bytes");
    assert_eq!(stream.len(), flight.len());
    assert!(stream == flight, "every byte, in order");
    assert!(polls <= 5, "took {polls} polls");
}

/// The same flight landing over several seconds, the way mix delays spread
/// it: every answer carries all that is in order by then, so nothing waits
/// in the proxy while the browser is told there is nothing.
#[test]
fn a_flight_arriving_over_time_is_handed_on_as_it_becomes_readable() {
    let flight = pattern(64 * 1600);
    let mut landing: Vec<(i64, Vec<u8>)> = flight
        .chunks(1600)
        .enumerate()
        .map(|(seq, chunk)| (0, response(1, seq as u64, seq == 63, chunk)))
        .collect();
    shuffle(&mut landing, 0x5eed);
    for (i, slot) in landing.iter_mut().enumerate() {
        slot.0 = 1_000 + (i as i64) * 50;
    }
    let mut net = Net::new(landing);
    let mut inbox = Inbox::default();
    let mut stream = Vec::new();
    for _ in 0..1_000 {
        let got = gather(&mut inbox, 1, ANSWER_MAX, HOLD_MS, &mut net, file);
        stream.extend_from_slice(&got.bytes);
        // Whatever could be read in order was read: draining again at once
        // finds nothing, unless the answer was full.
        if got.bytes.len() < ANSWER_MAX && !got.closed {
            assert!(inbox.drain(1, ANSWER_MAX).0.is_empty(), "a readable byte was left behind");
        }
        if got.closed {
            break;
        }
        net.now += 16;
    }
    assert!(stream == flight, "every byte, in order");
}

#[test]
fn a_gap_holds_back_only_what_is_behind_it() {
    let landing: Vec<(i64, Vec<u8>)> = (0..10u64)
        .filter(|seq| *seq != 3)
        .map(|seq| (0, response(1, seq, false, &[seq as u8; 10])))
        .collect();
    let mut net = Net::new(landing);
    let mut inbox = Inbox::default();
    let got = gather(&mut inbox, 1, ANSWER_MAX, HOLD_MS, &mut net, file);
    assert_eq!(got.bytes, [[0u8; 10], [1; 10], [2; 10]].concat());

    net.landing.push_back((net.now, response(1, 3, false, &[3; 10])));
    let got = gather(&mut inbox, 1, ANSWER_MAX, HOLD_MS, &mut net, file);
    let rest: Vec<u8> = (3..10u8).flat_map(|s| [s; 10]).collect();
    assert_eq!(got.bytes, rest, "the gap filling releases everything behind it at once");
}

#[test]
fn an_answer_never_carries_more_than_its_room() {
    let landing: Vec<(i64, Vec<u8>)> =
        (0..40u64).map(|seq| (0, response(1, seq, false, &[seq as u8; 1600]))).collect();
    let mut net = Net::new(landing);
    let mut inbox = Inbox::default();
    for room in [1usize, 1599, 1600, 1601, 5000] {
        let got = gather(&mut inbox, 1, room, HOLD_MS, &mut net, file);
        assert_eq!(got.bytes.len(), room, "full up to the room and no further");
    }
}

#[test]
fn an_answer_waits_only_when_it_has_nothing_to_give() {
    let mut net = Net::new(vec![(10, response(1, 0, false, b"late"))]);
    let mut inbox = Inbox::default();
    let got = gather(&mut inbox, 1, ANSWER_MAX, HOLD_MS, &mut net, file);
    assert_eq!(got.bytes, b"late");
    assert_eq!(net.waits.first(), Some(&(HOLD_MS as u32)), "nothing in hand, so it waits");
    assert!(net.waits[1..].iter().all(|w| *w == 0), "bytes in hand, so later reads do not");
}

#[test]
fn an_answer_with_nothing_comes_back_within_its_hold() {
    let mut net = Net::new(Vec::new());
    let mut inbox = Inbox::default();
    let got = gather(&mut inbox, 1, ANSWER_MAX, HOLD_MS, &mut net, file);
    assert!(got.bytes.is_empty() && !got.closed);
    assert!(net.now <= HOLD_MS, "held {} ms", net.now);
}

#[test]
fn bytes_for_another_stream_are_kept_for_it() {
    let landing = vec![(0, response(2, 0, false, b"for two")), (0, response(1, 0, false, b"one"))];
    let mut net = Net::new(landing);
    let mut inbox = Inbox::default();
    assert_eq!(gather(&mut inbox, 1, ANSWER_MAX, HOLD_MS, &mut net, file).bytes, b"one");
    assert_eq!(gather(&mut inbox, 2, ANSWER_MAX, HOLD_MS, &mut net, file).bytes, b"for two");
}

/// Another stream that never stops sending must not keep this poll from
/// being answered.
#[test]
fn a_busy_neighbour_cannot_hold_an_answer_back() {
    struct Endless {
        seq: u64,
        reads: usize,
    }
    impl Mixnet for Endless {
        fn receive(&mut self, _: u32) -> Received {
            self.reads += 1;
            self.seq += 1;
            Received::Messages(vec![response(2, self.seq, false, b"noise")])
        }
        fn now_ms(&self) -> i64 {
            0
        }
    }
    let mut net = Endless { seq: 0, reads: 0 };
    let mut inbox = Inbox::default();
    let got = gather(&mut inbox, 1, ANSWER_MAX, HOLD_MS, &mut net, file);
    assert!(got.bytes.is_empty());
    assert_eq!(net.reads, PULLS_MAX, "the answer goes back after a bounded number of reads");
}

#[test]
fn a_transport_that_is_gone_ends_the_answer_with_what_is_in_hand() {
    struct Gone;
    impl Mixnet for Gone {
        fn receive(&mut self, _: u32) -> Received {
            Received::Gone
        }
        fn now_ms(&self) -> i64 {
            0
        }
    }
    let mut inbox = Inbox::default();
    let _ = inbox.accept(1, 0, false, b"kept");
    let got = gather(&mut inbox, 1, ANSWER_MAX, HOLD_MS, &mut Gone, file);
    assert_eq!(got.bytes, b"kept");
    assert!(got.gone, "and the caller is told, so it can end every stream on it");
    assert!(!got.closed, "this stream did not finish; the transport went");
}

#[test]
fn a_stream_that_is_still_open_is_not_reported_gone() {
    let mut net = Net::new(vec![(0, response(1, 0, false, b"x"))]);
    let mut inbox = Inbox::default();
    let got = gather(&mut inbox, 1, ANSWER_MAX, HOLD_MS, &mut net, file);
    assert!(!got.gone && !got.closed);
}

/// Whatever arrives off the mixnet, a reply header that does not hold
/// together is refused without a panic, and the bytes a response hands on
/// always lie inside what arrived.
#[test]
fn arbitrary_replies_never_panic_and_never_overrun() {
    let mut state = 0x0bad_cafe_u64;
    let mut inbox = Inbox::default();
    for step in 0..20_000u32 {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let len = (state >> 58) as usize;
        let mut msg: Vec<u8> =
            (0..len).map(|i| (state.rotate_left(i as u32 * 5) >> 7) as u8).collect();
        // Half carry a believable envelope so the header paths run as well.
        if step % 2 == 0 && msg.len() >= 4 {
            msg[..4].copy_from_slice(&[3, 1, PROTOCOL_VERSION, RESP_NETWORK_DATA]);
        }
        if let Some(r) = decode_response(&msg) {
            assert!(r.data.len() <= msg.len());
        }
        file(&mut inbox, &msg);
        let _ = inbox.drain(1, ANSWER_MAX);
    }
}
