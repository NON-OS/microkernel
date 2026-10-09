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

//! Proofs that a reply split across packets comes back whole whatever the
//! mixnet did to its pieces: in order, reversed, shuffled, or interleaved
//! with dozens of other replies; that a copy of a piece is never counted
//! twice or taken for a new message; that a set missing a piece is given up
//! once it goes stale; and that no header, however malformed or however
//! large it claims to be, makes the pool grow past its bounds or panic.

use crate::message::{pad_to_packets, Fragment};
use crate::reply::{
    reply_body, reply_message, Collected, Reassembly, Reply, MAX_PENDING, MESSAGE_MAX,
    PENDING_BYTES_MAX, STALE_MS, TAG_REPLY_DATA, TAG_REPLY_SURB_REQUEST, TYPE_REPLY,
};

/// What one regular packet carries of a reply, once the acknowledgement, the
/// key digest and the fragment header are taken: the width an exit splits
/// its answers at.
const WIDTH: usize = 1604;

/// The fragments an exit would send `body` back as.
fn fragments_of(body: &[u8], set_id: i32, width: usize) -> Vec<Vec<u8>> {
    let mut message = vec![TYPE_REPLY, TAG_REPLY_DATA];
    message.extend_from_slice(body);
    let padded = pad_to_packets(message, width).expect("a real width pads");
    let total = (padded.len() / width) as u8;
    padded
        .chunks(width)
        .enumerate()
        .map(|(i, chunk)| Fragment { set_id, total, current: (i + 1) as u8 }.into_bytes(chunk))
        .collect()
}

/// A body whose every byte says where it sits, so a piece placed wrong or
/// lost shows up as a mismatch rather than as equal-looking filler.
fn body(len: usize, salt: u8) -> Vec<u8> {
    (0..len).map(|i| (i as u32).wrapping_mul(31).wrapping_add(salt as u32) as u8).collect()
}

/// A small deterministic generator, so a shuffled order is the same on every
/// run and a failure can be replayed.
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }

    fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            let j = (self.next() % (i as u64 + 1)) as usize;
            items.swap(i, j);
        }
    }
}

/// Feed every piece, returning each message completed along the way.
fn feed(pool: &mut Reassembly, pieces: &[Vec<u8>], now_ms: i64) -> Vec<Vec<u8>> {
    let mut done = Vec::new();
    for piece in pieces {
        if let Collected::Complete(message) = pool.collect(piece, now_ms) {
            done.push(message);
        }
    }
    done
}

fn only_body(done: &[Vec<u8>]) -> Vec<u8> {
    assert_eq!(done.len(), 1, "exactly one message completes");
    reply_body(&done[0]).expect("a reply carrying data").to_vec()
}

#[test]
fn a_reply_in_order_comes_back_whole() {
    let sent = body(5000, 1);
    let pieces = fragments_of(&sent, 7, WIDTH);
    assert!(pieces.len() > 2);
    let mut pool = Reassembly::new();
    assert_eq!(only_body(&feed(&mut pool, &pieces, 0)), sent);
    assert_eq!(pool.pending(), 0);
    assert_eq!(pool.held_bytes(), 0, "nothing is held once the message is out");
}

#[test]
fn a_reply_reversed_comes_back_whole() {
    let sent = body(5000, 2);
    let mut pieces = fragments_of(&sent, 8, WIDTH);
    pieces.reverse();
    let mut pool = Reassembly::new();
    assert_eq!(only_body(&feed(&mut pool, &pieces, 0)), sent);
}

#[test]
fn a_reply_shuffled_comes_back_whole() {
    let mut rng = Lcg(0x5eed);
    for round in 0..20u8 {
        let sent = body(3000 + round as usize * 977, round);
        let mut pieces = fragments_of(&sent, 100 + round as i32, WIDTH);
        rng.shuffle(&mut pieces);
        let mut pool = Reassembly::new();
        assert_eq!(only_body(&feed(&mut pool, &pieces, 0)), sent, "round {round}");
    }
}

/// A 100 KB answer in one message spans sixty-odd packets; every one of
/// them, in whatever order, has to land in its own place.
#[test]
fn a_hundred_kilobyte_reply_in_one_message_comes_back_whole() {
    let sent = body(100 * 1024, 9);
    let mut pieces = fragments_of(&sent, 4242, WIDTH);
    assert!(pieces.len() >= 64);
    Lcg(77).shuffle(&mut pieces);
    let mut pool = Reassembly::new();
    assert_eq!(only_body(&feed(&mut pool, &pieces, 0)), sent);
}

#[test]
fn a_repeated_fragment_is_not_counted_twice() {
    let pieces = fragments_of(&body(4000, 3), 11, WIDTH);
    assert!(pieces.len() > 2);
    let mut pool = Reassembly::new();
    assert!(matches!(pool.collect(&pieces[0], 0), Collected::Held));
    for _ in 0..pieces.len() * 2 {
        assert!(
            matches!(pool.collect(&pieces[0], 0), Collected::Duplicate),
            "a copy of a placed piece is a copy, not progress"
        );
    }
    assert_eq!(pool.pending(), 1);
}

/// The far end resends a fragment whose acknowledgement was slow, and the
/// copy can land after its message was already delivered. Taking it for the
/// start of a new message delivered a one-packet answer twice, which puts a
/// chunk of the stream in front of the reader a second time.
#[test]
fn a_copy_arriving_after_its_message_is_not_a_second_message() {
    let sent = body(200, 4);
    let pieces = fragments_of(&sent, 12, WIDTH);
    assert_eq!(pieces.len(), 1);
    let mut pool = Reassembly::new();
    assert_eq!(only_body(&feed(&mut pool, &pieces, 0)), sent);
    assert!(
        matches!(pool.collect(&pieces[0], 10), Collected::Duplicate),
        "the same set again is a copy of what was delivered"
    );
}

/// A late copy of one piece of a delivered message used to open a new set
/// that could never complete. Each such shell held a slot, and enough of
/// them evicted live sets that were one piece from done.
#[test]
fn a_late_copy_does_not_take_a_slot() {
    let pieces = fragments_of(&body(4000, 5), 13, WIDTH);
    let mut pool = Reassembly::new();
    assert_eq!(feed(&mut pool, &pieces, 0).len(), 1);
    for piece in &pieces {
        assert!(matches!(pool.collect(piece, 5), Collected::Duplicate));
    }
    assert_eq!(pool.pending(), 0, "no shell was opened for the copies");
}

/// An exit answers a TLS flight as dozens of messages sent back to back, and
/// their packets arrive interleaved. With room for only eight half-built
/// messages the ninth evicted the first, whose missing piece then arrived to
/// nothing; the stream behind it waited forever.
#[test]
fn dozens_of_interleaved_replies_all_complete() {
    let count = 48;
    let sent: Vec<Vec<u8>> = (0..count).map(|i| body(4000, i as u8)).collect();
    let sets: Vec<Vec<Vec<u8>>> =
        sent.iter().enumerate().map(|(i, b)| fragments_of(b, 1000 + i as i32, WIDTH)).collect();
    let per = sets[0].len();
    assert!(per >= 3);
    let mut pool = Reassembly::new();
    let mut done = Vec::new();
    // Every first piece, then every second, and so on: the widest spread
    // of half-built messages a burst can produce.
    for at in 0..per {
        for set in &sets {
            if let Collected::Complete(message) = pool.collect(&set[at], at as i64) {
                done.push(reply_body(&message).expect("a data reply").to_vec());
            }
        }
    }
    assert_eq!(done.len(), count, "every message completes");
    assert_eq!(done, sent, "and they complete in the order their last piece came");
}

#[test]
fn a_set_missing_a_piece_is_abandoned_once_stale() {
    let pieces = fragments_of(&body(4000, 6), 14, WIDTH);
    assert_eq!(pieces.len(), 3);
    let mut pool = Reassembly::new();
    assert!(matches!(pool.collect(&pieces[0], 0), Collected::Held));
    assert!(matches!(pool.collect(&pieces[2], 1_000), Collected::Held));
    assert!(pool.held_bytes() > 0);

    // Nothing for longer than the far end would wait on an acknowledgement.
    let later = 1_000 + STALE_MS + 1;
    let other = fragments_of(&body(4000, 7), 15, WIDTH);
    assert!(matches!(pool.collect(&other[0], later), Collected::Held));
    assert_eq!(pool.pending(), 1, "only the fresh set is left");
    assert_eq!(pool.held_bytes(), other[0].len() - crate::message::UNLINKED_HEADER_LEN);

    // The missing piece turning up now starts over rather than completing a
    // message out of pieces that are gone.
    assert!(matches!(pool.collect(&pieces[1], later + 1), Collected::Held));
}

#[test]
fn a_set_still_hearing_pieces_is_not_abandoned() {
    let pieces = fragments_of(&body(8000, 8), 16, WIDTH);
    assert!(pieces.len() >= 5);
    let mut pool = Reassembly::new();
    let mut done = Vec::new();
    // Slow but steady: each piece lands just inside the stale window of the
    // one before, so the set as a whole takes far longer than the window.
    for (i, piece) in pieces.iter().enumerate() {
        if let Collected::Complete(m) = pool.collect(piece, i as i64 * (STALE_MS - 1)) {
            done.push(m);
        }
    }
    assert_eq!(done.len(), 1, "progress keeps a set alive");
}

/// A header claiming 255 pieces of four kilobytes asks for a megabyte before
/// a second piece has arrived. It is refused at the first piece.
#[test]
fn a_set_claiming_more_than_a_message_may_hold_is_refused() {
    let width = 4096;
    let claimed = Fragment { set_id: 17, total: 255, current: 1 }.into_bytes(&vec![0xAB; width]);
    assert!(255 * width > MESSAGE_MAX);
    let mut pool = Reassembly::new();
    assert!(matches!(pool.collect(&claimed, 0), Collected::Refused));
    assert_eq!(pool.pending(), 0);
    assert_eq!(pool.held_bytes(), 0);
}

#[test]
fn a_piece_naming_another_size_for_a_set_under_way_is_refused() {
    let mut pool = Reassembly::new();
    let first = Fragment { set_id: 18, total: 3, current: 1 }.into_bytes(&[1; 64]);
    let liar = Fragment { set_id: 18, total: 2, current: 2 }.into_bytes(&[2; 64]);
    assert!(matches!(pool.collect(&first, 0), Collected::Held));
    assert!(matches!(pool.collect(&liar, 0), Collected::Refused));
    assert_eq!(pool.pending(), 1);
}

#[test]
fn the_pool_never_names_more_sets_than_its_bound() {
    let mut pool = Reassembly::new();
    for id in 0..1000 {
        let piece = Fragment { set_id: id, total: 2, current: 1 }.into_bytes(&[0; 32]);
        let _ = pool.collect(&piece, id as i64);
        assert!(pool.pending() <= MAX_PENDING);
    }
    assert_eq!(pool.pending(), MAX_PENDING);
}

#[test]
fn the_pool_never_holds_more_bytes_than_its_bound() {
    let mut pool = Reassembly::new();
    let width = 32 * 1024;
    for id in 0..400 {
        // Fifteen pieces of 32 KiB is just under the per-message bound, so
        // each set is admissible on its own and only the pool bound stops
        // them piling up.
        let piece = Fragment { set_id: id, total: 15, current: 1 + (id % 14) as u8 }
            .into_bytes(&vec![id as u8; width]);
        let _ = pool.collect(&piece, id as i64);
        assert!(pool.held_bytes() <= PENDING_BYTES_MAX, "after set {id}");
    }
    assert!(pool.held_bytes() > PENDING_BYTES_MAX / 2, "the bound is used, not avoided");
}

/// When the pool is full the set abandoned is the one heard from least
/// recently, not the one opened first: a set still receiving pieces is the
/// one closest to completing.
#[test]
fn the_set_abandoned_is_the_one_heard_from_least_recently() {
    let mut pool = Reassembly::new();
    let keeper = fragments_of(&body(4000, 9), 5000, WIDTH);
    assert_eq!(keeper.len(), 3);
    assert!(matches!(pool.collect(&keeper[0], 0), Collected::Held));
    for id in 1..MAX_PENDING as i32 {
        let piece = Fragment { set_id: id, total: 2, current: 1 }.into_bytes(&[0; 32]);
        assert!(matches!(pool.collect(&piece, id as i64), Collected::Held));
    }
    // The keeper hears again after everything else, then a newcomer needs a
    // slot: set 1, the stalest, is the one to go.
    assert!(matches!(pool.collect(&keeper[1], 1_000), Collected::Held));
    let newcomer = Fragment { set_id: 9999, total: 2, current: 1 }.into_bytes(&[0; 32]);
    assert!(matches!(pool.collect(&newcomer, 1_001), Collected::Held));
    assert_eq!(pool.pending(), MAX_PENDING);
    let done = feed(&mut pool, &keeper[2..], 1_002);
    assert_eq!(done.len(), 1, "the set that was nearly done survived to finish");
    let gone = Fragment { set_id: 1, total: 2, current: 2 }.into_bytes(&[0; 32]);
    assert!(
        matches!(pool.collect(&gone, 1_003), Collected::Held),
        "set 1 was the one abandoned, so its second piece starts over"
    );
}

#[test]
fn malformed_fragment_headers_are_refused() {
    let mut pool = Reassembly::new();
    let good = Fragment { set_id: 19, total: 2, current: 1 }.into_bytes(&[7; 16]);
    let mut cases: Vec<Vec<u8>> = vec![vec![], good[..6].to_vec()];
    // No marker bit: not a fragment header at all.
    let mut unmarked = good.clone();
    unmarked[0] &= 0x7f;
    cases.push(unmarked);
    // Zero pieces, position zero, a position past the end.
    for (total, current) in [(0u8, 0u8), (2, 0), (2, 3), (0, 1)] {
        let mut bad = good.clone();
        bad[4] = total;
        bad[5] = current;
        cases.push(bad);
    }
    // A linked set, which this neither sends nor reads.
    let mut linked = good.clone();
    linked[6] = 1;
    cases.push(linked);
    for case in &cases {
        assert!(matches!(pool.collect(case, 0), Collected::Refused), "{case:?}");
    }
    assert_eq!(pool.pending(), 0);
}

#[test]
fn arbitrary_bytes_never_panic_or_escape_the_bounds() {
    let mut rng = Lcg(0xfeed);
    let mut pool = Reassembly::new();
    for step in 0..20_000i64 {
        let len = (rng.next() % 64) as usize;
        let mut bytes: Vec<u8> = (0..len).map(|_| rng.next() as u8).collect();
        // Half of them get a plausible header so the deeper paths run too.
        if len >= 7 && step % 2 == 0 {
            bytes[0] |= 0x80;
            bytes[4] = 1 + (rng.next() % 4) as u8;
            bytes[5] = 1 + (rng.next() % bytes[4] as u64) as u8;
            bytes[6] = 0;
        }
        let _ = pool.collect(&bytes, step);
        assert!(pool.pending() <= MAX_PENDING);
        assert!(pool.held_bytes() <= PENDING_BYTES_MAX);
    }
}

#[test]
fn a_reassembled_request_for_more_reply_blocks_is_read() {
    let mut message = vec![TYPE_REPLY, TAG_REPLY_SURB_REQUEST];
    message.extend_from_slice(&[0x11; 96]);
    message.extend_from_slice(&100u32.to_be_bytes());
    let padded = pad_to_packets(message, WIDTH).expect("pads");
    match reply_message(&padded) {
        Some(Reply::SurbRequest { recipient, amount }) => {
            assert_eq!(recipient, [0x11; 96]);
            assert_eq!(amount, 100);
        }
        _ => panic!("a request for reply blocks must be read as one"),
    }
}

#[test]
fn a_reassembled_message_that_is_not_a_reply_is_not_read() {
    for message in [
        vec![TYPE_REPLY + 1, TAG_REPLY_DATA, 1, 2, 3],
        vec![TYPE_REPLY, 9, 1, 2, 3],
        vec![TYPE_REPLY],
        // A request for blocks too short to name where to send them.
        vec![TYPE_REPLY, TAG_REPLY_SURB_REQUEST, 0, 0, 0],
    ] {
        let padded = pad_to_packets(message, 64).expect("pads");
        assert!(reply_message(&padded).is_none());
    }
    // No padding marker at all.
    assert!(reply_message(&[0u8; 64]).is_none());
}

#[test]
fn arbitrary_reassembled_messages_never_panic() {
    let mut rng = Lcg(0xabad1dea);
    for _ in 0..20_000 {
        let len = (rng.next() % 160) as usize;
        let mut bytes: Vec<u8> = (0..len).map(|_| rng.next() as u8).collect();
        if len > 2 && rng.next().is_multiple_of(2) {
            bytes[0] = TYPE_REPLY;
            bytes[1] = (rng.next() % 3) as u8;
        }
        match reply_message(&bytes) {
            Some(Reply::Data(body)) => assert!(body.len() <= bytes.len()),
            Some(Reply::SurbRequest { .. }) | None => {}
        }
    }
}
