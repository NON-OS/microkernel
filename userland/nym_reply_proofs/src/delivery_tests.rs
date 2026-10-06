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

//! Proofs that what net.nym has queued reaches net.socks5 whole: many
//! messages in one answer, a message longer than an answer in pieces that
//! are put back together and never read as wholes, a message that would be
//! split behind others left whole for the next answer, and the queue held
//! to its bounds without ever gluing one message to the next. Both ends are
//! the real sources: net.nym's queue writes the records, net.socks5's joiner
//! reads them.

use crate::protocol::batch::{FLAG_CONT, FLAG_MORE, RECORD_HEADER};
use crate::socks5_batch::Joiner;
use crate::state::rx_queue::{RxQueue, RX_BYTES_MAX, RX_DEPTH};

/// What one batched answer carries after the 20 byte IPC header: net.nym
/// answers into a 32 KiB payload.
const ANSWER: usize = 32 * 1024;

fn message(len: usize, salt: u8) -> Vec<u8> {
    (0..len).map(|i| (i as u32).wrapping_mul(13).wrapping_add(salt as u32) as u8).collect()
}

/// Read answers until the queue is empty, returning every message the far
/// side put back together and how many answers it took.
fn collect_all(queue: &mut RxQueue, joiner: &mut Joiner, answer: usize) -> (Vec<Vec<u8>>, usize) {
    let mut out = Vec::new();
    let mut answers = 0;
    while !queue.is_empty() {
        let mut buf = vec![0u8; answer];
        let n = queue.fill_records(&mut buf);
        assert!(n > 0, "a queue with something in it always gives something");
        answers += 1;
        let got = joiner.feed(&buf[..n]);
        assert_eq!(got.dropped, 0);
        assert!(!got.malformed);
        out.extend(got.messages);
    }
    (out, answers)
}

#[test]
fn many_messages_go_back_in_one_answer() {
    let mut queue = RxQueue::new();
    let sent: Vec<Vec<u8>> = (0..40).map(|i| message(700, i)).collect();
    for m in &sent {
        assert!(queue.push(m.clone()));
    }
    let (got, answers) = collect_all(&mut queue, &mut Joiner::new(), ANSWER);
    assert_eq!(got, sent);
    assert_eq!(answers, 1, "one round trip, not forty");
}

/// A message longer than one answer went out as its front, and its rest as
/// a separate read the proxy took for a message of its own. The front read
/// as a response with its data cut short, the rest did not read at all, and
/// the stream lost everything past the cut.
#[test]
fn a_message_longer_than_an_answer_arrives_whole_across_answers() {
    let mut queue = RxQueue::new();
    let long = message(100 * 1024, 1);
    let after = message(300, 2);
    queue.push(long.clone());
    queue.push(after.clone());
    let (got, answers) = collect_all(&mut queue, &mut Joiner::new(), ANSWER);
    assert_eq!(got.len(), 2);
    assert!(got[0] == long, "the long message is put back together byte for byte");
    assert_eq!(got[1], after);
    assert!(answers >= 4);
}

#[test]
fn the_pieces_say_which_they_are() {
    let mut queue = RxQueue::new();
    queue.push(message(10_000, 3));
    let mut buf = vec![0u8; 4_000];
    let mut flags = Vec::new();
    let mut carried = 0;
    while !queue.is_empty() {
        let n = queue.fill_records(&mut buf);
        flags.push(buf[0]);
        carried += n - RECORD_HEADER;
        if !queue.is_empty() {
            assert_eq!(n, buf.len(), "every piece but the last fills its answer");
        }
    }
    assert_eq!(carried, 10_000);
    assert_eq!(flags.first(), Some(&FLAG_MORE), "the front says more follows");
    assert_eq!(flags.last(), Some(&FLAG_CONT), "the back says it continues and ends");
    assert!(flags[1..flags.len() - 1].iter().all(|f| *f == FLAG_CONT | FLAG_MORE));
}

/// A message that does not fit behind the ones already in an answer waits
/// whole for the next, rather than going as a sliver at the end of this one.
#[test]
fn a_message_that_does_not_fit_behind_others_waits_whole() {
    let mut queue = RxQueue::new();
    let big = ANSWER - RECORD_HEADER - 100;
    queue.push(message(1_000, 4));
    queue.push(message(big, 5));
    let mut buf = vec![0u8; ANSWER];
    let n = queue.fill_records(&mut buf);
    assert_eq!(n, RECORD_HEADER + 1_000);
    let n = queue.fill_records(&mut buf);
    assert_eq!(n, RECORD_HEADER + big);
    assert_eq!(buf[0], 0, "whole, with no flags");
}

#[test]
fn an_answer_never_runs_past_its_buffer() {
    for room in [0usize, 1, RECORD_HEADER, RECORD_HEADER + 1, 100, 4097] {
        let mut queue = RxQueue::new();
        for i in 0..5 {
            queue.push(message(1_500, i));
        }
        let mut buf = vec![0u8; room];
        let n = queue.fill_records(&mut buf);
        assert!(n <= room);
        if room <= RECORD_HEADER {
            assert_eq!(n, 0, "no room for a header and a byte");
            assert_eq!(queue.len(), 5, "and nothing was taken");
        }
    }
}

#[test]
fn the_queue_is_held_to_its_count_and_its_bytes() {
    let mut queue = RxQueue::new();
    for i in 0..(RX_DEPTH * 2) {
        queue.push(message(16, i as u8));
        assert!(queue.len() <= RX_DEPTH);
    }
    let mut queue = RxQueue::new();
    for i in 0..200 {
        queue.push(message(64 * 1024, i));
        assert!(queue.bytes() <= RX_BYTES_MAX);
    }
    assert!(!queue.push(vec![0u8; RX_BYTES_MAX + 1]), "a message larger than the queue is refused");
}

/// When the queue drops the rest of a message whose front already went, the
/// reader must not glue the next message onto that front.
#[test]
fn a_lost_rest_never_joins_the_front_to_the_next_message() {
    let mut queue = RxQueue::new();
    let mut joiner = Joiner::new();
    queue.push(message(40 * 1024, 6));
    let mut buf = vec![0u8; ANSWER];
    let n = queue.fill_records(&mut buf);
    assert!(joiner.feed(&buf[..n]).messages.is_empty(), "only the front so far");
    // The reader stops reading; the queue fills and drops its oldest, which
    // is the rest of that message.
    for i in 0..RX_DEPTH {
        queue.push(message(100, i as u8));
    }
    let n = queue.fill_records(&mut buf);
    let got = joiner.feed(&buf[..n]);
    assert_eq!(got.dropped, 1, "the unfinished front is dropped");
    assert!(got.messages.iter().all(|m| m.len() == 100), "every message handed on is whole");
}
