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

//! Proofs that a reply block's key is kept until its reply arrives however
//! many blocks were handed out after it, and opens one reply only; and that
//! the budget of blocks the far end holds is replenished ahead of need, so a
//! 100 KB answer and a long download both keep flowing instead of stopping
//! at the far end's reserve for a round trip each time.

use crate::surb::budget::{
    Budget, ANSWER_MAX, ANSWER_MIN, HIGH_WATER, LOW_WATER, PER_REQUEST, RESERVE, SPACING_MS, TOP_UP,
};
use crate::surb::store::{KeyStore, DIGEST_BYTES, KEYS_MAX};
use crate::surb::types::SURB_KEY_BYTES;

fn key(n: u32) -> [u8; SURB_KEY_BYTES] {
    let mut k = [0u8; SURB_KEY_BYTES];
    k[..4].copy_from_slice(&n.to_le_bytes());
    k[15] = 0xA5;
    k
}

/// A stand-in digest; the store matches on whatever the caller computed.
fn digest(n: u32) -> [u8; DIGEST_BYTES] {
    let mut d = [0u8; DIGEST_BYTES];
    d[..4].copy_from_slice(&n.wrapping_mul(2_654_435_761).to_be_bytes());
    d[31] = 0x5A;
    d
}

#[test]
fn a_reply_finds_the_key_its_digest_names() {
    let mut store = KeyStore::new();
    for n in 0..50 {
        store.remember(key(n), digest(n));
    }
    assert_eq!(store.take(&digest(17)), Some(key(17)));
    assert_eq!(store.take(&[0u8; DIGEST_BYTES]), None, "a digest of nothing we handed out");
    assert_eq!(store.take(&digest(17)[..16]), None, "a short digest matches nothing");
}

#[test]
fn a_key_opens_one_reply_only() {
    let mut store = KeyStore::new();
    store.remember(key(1), digest(1));
    assert_eq!(store.take(&digest(1)), Some(key(1)));
    assert_eq!(store.take(&digest(1)), None, "a second reply on one block is a copy");
    assert!(store.is_empty());
}

/// The far end spends the blocks it holds oldest first. A ring of the newest
/// 512 keys had dropped the key of an early block by the time a page had
/// made a couple of dozen requests, and the reply that came back on it was
/// dropped as addressed to somebody else.
#[test]
fn a_block_handed_out_long_ago_still_opens() {
    let mut store = KeyStore::new();
    for n in 0..2_000 {
        store.remember(key(n), digest(n));
    }
    assert_eq!(store.take(&digest(0)), Some(key(0)), "the oldest unanswered block");
    assert_eq!(store.len(), 1_999);
}

#[test]
fn the_store_is_bounded_and_lets_go_of_the_oldest() {
    let mut store = KeyStore::new();
    let total = KEYS_MAX as u32 + 10;
    for n in 0..total {
        store.remember(key(n), digest(n));
        assert!(store.len() <= KEYS_MAX);
    }
    for n in 0..10 {
        assert_eq!(store.take(&digest(n)), None, "the oldest ten went");
    }
    assert_eq!(store.take(&digest(10)), Some(key(10)));
    assert_eq!(store.take(&digest(total - 1)), Some(key(total - 1)));
}

#[test]
fn a_request_carries_blocks_until_the_far_end_holds_plenty() {
    let mut budget = Budget::new();
    assert_eq!(budget.for_request(), PER_REQUEST);
    while budget.held() + PER_REQUEST <= HIGH_WATER {
        let n = budget.for_request();
        assert_eq!(n, PER_REQUEST);
        budget.sent(n);
    }
    let n = budget.for_request();
    assert_eq!(n, HIGH_WATER - budget.held(), "just enough to reach the mark");
    budget.sent(n);
    assert_eq!(budget.for_request(), 0, "no more once it holds plenty");
    budget.spent();
    assert_eq!(budget.for_request(), 1);
}

#[test]
fn every_packet_back_spends_one_and_the_count_never_wraps() {
    let mut budget = Budget::new();
    budget.sent(3);
    for _ in 0..10 {
        budget.spent();
    }
    assert_eq!(budget.held(), 0);
}

#[test]
fn a_request_for_more_is_answered_with_room_to_spare() {
    let mut budget = Budget::new();
    budget.sent(250);
    assert_eq!(budget.asked(90), 90);
    assert_eq!(budget.held(), RESERVE, "it only asks once down to its reserve");
    assert_eq!(budget.asked(1), ANSWER_MIN, "a small ask still leaves room");
    assert_eq!(budget.asked(5_000), ANSWER_MAX, "and a greedy one is capped");
}

#[test]
fn blocks_go_ahead_only_when_running_low_and_not_twice_at_once() {
    let mut budget = Budget::new();
    budget.sent(LOW_WATER);
    assert_eq!(budget.top_up_due(0), None, "not low yet");
    budget.spent();
    assert_eq!(budget.top_up_due(0), Some(TOP_UP));
    budget.topped_up(TOP_UP, 0);
    for _ in 0..TOP_UP {
        budget.spent();
    }
    assert_eq!(budget.top_up_due(SPACING_MS - 1), None, "the last one may still be on its way");
    assert_eq!(budget.top_up_due(SPACING_MS), Some(TOP_UP));
}

/// One way across the mixnet.
const LEG_MS: i64 = 1_000;
/// How often the far end puts a reply packet on the wire.
const PACE_MS: i64 = 20;
/// How long the far end waits on a request for blocks before asking again.
const REASK_MS: i64 = 10_000;
/// Simulation step.
const STEP_MS: i64 = 10;

/// What the client does about blocks: the policy under test, or the one it
/// replaced.
trait Policy {
    fn for_request(&mut self) -> u32;
    /// A data packet came back at `now`; returns blocks to send ahead.
    fn on_data(&mut self, now: i64) -> u32;
    /// The far end asked for `amount`; returns blocks to send.
    fn on_ask(&mut self, amount: u32) -> u32;
}

struct Current(Budget);

impl Policy for Current {
    fn for_request(&mut self) -> u32 {
        let n = self.0.for_request();
        self.0.sent(n);
        n
    }
    fn on_data(&mut self, now: i64) -> u32 {
        self.0.spent();
        match self.0.top_up_due(now) {
            Some(n) => {
                self.0.topped_up(n, now);
                n
            }
            None => 0,
        }
    }
    fn on_ask(&mut self, amount: u32) -> u32 {
        self.0.spent();
        let n = self.0.asked(amount);
        self.0.sent(n);
        n
    }
}

/// net.nym before: 24 blocks a request, nothing ahead, at most 40 asked.
struct Before;

impl Policy for Before {
    fn for_request(&mut self) -> u32 {
        24
    }
    fn on_data(&mut self, _: i64) -> u32 {
        0
    }
    fn on_ask(&mut self, amount: u32) -> u32 {
        amount.min(40)
    }
}

struct Outcome {
    done_ms: i64,
    asks: u32,
}

enum Up {
    Data,
    Ask(u32),
}

/// A requester answering `packets` packets after the client's `requests`
/// requests reach it, spending one block a packet, keeping `RESERVE` back
/// and asking for what its queue needs (ten to a hundred) once down to it.
fn run(policy: &mut impl Policy, requests: u32, packets: u32) -> Outcome {
    let mut down: Vec<(i64, u32)> = Vec::new();
    for _ in 0..requests {
        let n = policy.for_request();
        down.push((LEG_MS, n));
    }
    let mut up: Vec<(i64, Up)> = Vec::new();
    let (mut pool, mut pending, mut delivered, mut asks) = (0u32, packets, 0u32, 0u32);
    let mut next_send = LEG_MS;
    let mut asked_at: Option<i64> = None;
    let mut t = 0;
    while delivered < packets {
        assert!(t < 3_600_000, "the answer never finished");
        let mut i = 0;
        while i < down.len() {
            if down[i].0 <= t {
                pool += down.remove(i).1;
                asked_at = None;
            } else {
                i += 1;
            }
        }
        if pending > 0 && t >= next_send {
            if pool > RESERVE {
                pool -= 1;
                pending -= 1;
                up.push((t + LEG_MS, Up::Data));
                next_send = t + PACE_MS;
            } else if pool > 0 && asked_at.is_none_or(|at| t - at >= REASK_MS) {
                pool -= 1;
                asks += 1;
                asked_at = Some(t);
                up.push((t + LEG_MS, Up::Ask(pending.clamp(10, 100))));
                next_send = t + PACE_MS;
            }
        }
        let mut i = 0;
        while i < up.len() {
            if up[i].0 > t {
                i += 1;
                continue;
            }
            let back = match up.remove(i).1 {
                Up::Data => {
                    delivered += 1;
                    policy.on_data(t)
                }
                Up::Ask(amount) => policy.on_ask(amount),
            };
            if back > 0 {
                down.push((t + LEG_MS, back));
            }
        }
        t += STEP_MS;
    }
    Outcome { done_ms: t, asks }
}

/// The time the answer takes with blocks never in short supply: the
/// requests' crossing, the packets at the far end's pace, the last one's
/// crossing back.
fn unhindered(packets: u32) -> i64 {
    2 * LEG_MS + packets as i64 * PACE_MS
}

/// A 100 KB answer is about 128 packets, far past the 28 usable blocks two
/// requests carry. Before, the far end stopped at its reserve three times
/// and waited a full round trip each time; now blocks go ahead as soon as
/// answers start arriving, and it asks at most once.
#[test]
fn a_hundred_kilobyte_answer_waits_on_the_reserve_at_most_once() {
    let now = run(&mut Current(Budget::new()), 2, 128);
    let before = run(&mut Before, 2, 128);
    assert!(now.asks <= 1, "asked {} times", now.asks);
    assert!(before.asks >= 3, "the old policy asked {} times", before.asks);
    assert!(
        now.done_ms <= unhindered(128) + 2 * LEG_MS,
        "{} ms against {} unhindered",
        now.done_ms,
        unhindered(128)
    );
    assert!(now.done_ms < before.done_ms, "{} ms now, {} before", now.done_ms, before.done_ms);
}

/// A long download keeps flowing: once the blocks ahead are coming, the far
/// end never has to ask again, however long the answer.
#[test]
fn a_long_answer_keeps_flowing_without_asking_again() {
    for packets in [640u32, 3_000] {
        let now = run(&mut Current(Budget::new()), 2, packets);
        assert!(now.asks <= 1, "{packets} packets: asked {} times", now.asks);
        assert!(
            now.done_ms <= unhindered(packets) + 2 * LEG_MS,
            "{packets} packets: {} ms against {} unhindered",
            now.done_ms,
            unhindered(packets)
        );
    }
}

/// A small answer, the usual case, costs no extra blocks up front.
#[test]
fn a_small_answer_needs_nothing_extra() {
    let now = run(&mut Current(Budget::new()), 2, 12);
    assert_eq!(now.asks, 0);
    assert!(now.done_ms <= unhindered(12) + STEP_MS);
}
