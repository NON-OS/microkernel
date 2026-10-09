// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs that stream bytes coming back off the mixnet are put in order: a
//! chunk that arrives before the one in front of it is held rather than
//! passed on, chunks are handed over only once their gap is filled, two
//! connections do not see each other's bytes, a repeat of an already
//! delivered position is not delivered twice, the far end closing is reported
//! with the bytes that closed it, and forgetting a connection drops what was
//! still held for it.

use crate::inbox::Inbox;
use crate::reply::ANSWER_MAX;

#[test]
fn chunks_in_order_come_straight_back() {
    let mut inbox = Inbox::default();
    inbox.accept(1, 0, false, b"hello ");
    inbox.accept(1, 1, false, b"world");
    let (bytes, closed) = inbox.drain(1, ANSWER_MAX);
    assert_eq!(bytes, b"hello world");
    assert!(!closed);
}

#[test]
fn a_chunk_that_arrives_early_waits_for_the_gap() {
    let mut inbox = Inbox::default();
    // The mixnet delays every packet separately, so the second half can and
    // does land first.
    inbox.accept(1, 1, false, b"world");
    let (bytes, _) = inbox.drain(1, ANSWER_MAX);
    assert!(bytes.is_empty(), "nothing is handed over across a gap");

    inbox.accept(1, 0, false, b"hello ");
    let (bytes, _) = inbox.drain(1, ANSWER_MAX);
    assert_eq!(bytes, b"hello world", "the gap filling releases both");
}

#[test]
fn connections_do_not_see_each_others_bytes() {
    let mut inbox = Inbox::default();
    inbox.accept(1, 0, false, b"one");
    inbox.accept(2, 0, false, b"two");
    assert_eq!(inbox.drain(1, ANSWER_MAX).0, b"one");
    assert_eq!(inbox.drain(2, ANSWER_MAX).0, b"two");
}

#[test]
fn a_position_already_delivered_is_not_delivered_again() {
    let mut inbox = Inbox::default();
    inbox.accept(1, 0, false, b"abc");
    assert_eq!(inbox.drain(1, ANSWER_MAX).0, b"abc");
    // A retransmit, or a duplicate the mixnet delivered twice.
    inbox.accept(1, 0, false, b"abc");
    assert!(inbox.drain(1, ANSWER_MAX).0.is_empty(), "the stream has moved past it");
    inbox.accept(1, 1, false, b"def");
    assert_eq!(inbox.drain(1, ANSWER_MAX).0, b"def");
}

#[test]
fn a_close_is_reported_with_the_bytes_that_carried_it() {
    let mut inbox = Inbox::default();
    inbox.accept(1, 0, false, b"body");
    inbox.accept(1, 1, true, b" end");
    let (bytes, closed) = inbox.drain(1, ANSWER_MAX);
    assert_eq!(bytes, b"body end");
    assert!(closed, "the far end finished");
}

#[test]
fn a_close_behind_a_gap_is_not_reported_early() {
    let mut inbox = Inbox::default();
    inbox.accept(1, 1, true, b" end");
    let (bytes, closed) = inbox.drain(1, ANSWER_MAX);
    assert!(bytes.is_empty());
    assert!(!closed, "a close cannot overtake the bytes in front of it");
}

#[test]
fn forgetting_a_connection_drops_what_was_held() {
    let mut inbox = Inbox::default();
    inbox.accept(1, 1, false, b"world");
    inbox.forget(1);
    inbox.accept(1, 0, false, b"hello ");
    let (bytes, _) = inbox.drain(1, ANSWER_MAX);
    assert_eq!(bytes, b"hello ", "the held tail went with the connection");
}

#[test]
fn a_chunk_longer_than_the_room_is_split_and_its_tail_kept() {
    let mut inbox = Inbox::default();
    inbox.accept(1, 0, false, b"abcdef");
    inbox.accept(1, 1, true, b"gh");
    let (bytes, closed) = inbox.drain(1, 4);
    assert_eq!(bytes, b"abcd");
    assert!(!closed, "the close waits for the bytes in front of it");
    let (bytes, closed) = inbox.drain(1, 4);
    assert_eq!(bytes, b"efgh", "the tail first, then the next chunk");
    assert!(closed);
}

#[test]
fn no_room_takes_nothing_and_loses_nothing() {
    let mut inbox = Inbox::default();
    inbox.accept(1, 0, true, b"all");
    assert_eq!(inbox.drain(1, 0), (Vec::new(), false));
    assert_eq!(inbox.drain(1, ANSWER_MAX), (b"all".to_vec(), true));
}

use crate::inbox::{Accept, CONN_CHUNKS_MAX, CONN_HELD_MAX, GAP_MS, HELD_MAX};

#[test]
fn a_copy_of_a_position_already_read_is_not_held() {
    let mut inbox = Inbox::default();
    assert_eq!(inbox.accept(1, 0, false, b"abc"), Accept::Held);
    assert_eq!(inbox.drain(1, ANSWER_MAX).0, b"abc");
    // Held for ever before: the stream had moved past the only place it
    // could be read.
    assert_eq!(inbox.accept(1, 0, false, b"abc"), Accept::Duplicate);
    assert_eq!(inbox.held_bytes(), 0, "nothing is kept for it");
}

#[test]
fn a_copy_of_a_position_already_held_is_not_held_twice() {
    let mut inbox = Inbox::default();
    assert_eq!(inbox.accept(1, 2, false, b"later"), Accept::Held);
    assert_eq!(inbox.accept(1, 2, false, b"later"), Accept::Duplicate);
    assert_eq!(inbox.held_bytes(), 5);
    let _ = inbox.accept(1, 0, false, b"a");
    let _ = inbox.accept(1, 1, false, b"b");
    assert_eq!(inbox.drain(1, ANSWER_MAX).0, b"ablater");
    assert_eq!(inbox.held_bytes(), 0);
}

#[test]
fn a_reader_that_falls_too_far_behind_has_its_stream_ended() {
    let mut inbox = Inbox::default();
    let chunk = vec![7u8; 64 * 1024];
    let mut seq = 0;
    let ended = loop {
        match inbox.accept(1, seq, false, &chunk) {
            Accept::Held => seq += 1,
            other => break other,
        }
        assert!(seq as usize * chunk.len() <= CONN_HELD_MAX);
    };
    assert_eq!(ended, Accept::Overflow(1));
    assert_eq!(inbox.drain(1, ANSWER_MAX), (Vec::new(), true), "the reader is told it is over");
}

#[test]
fn a_stream_of_empty_messages_is_bounded_too() {
    let mut inbox = Inbox::default();
    // Seq 0 never comes, so nothing can be read and everything is held.
    let mut ended = None;
    for seq in 1..=(CONN_CHUNKS_MAX as u64 + 1) {
        if let Accept::Overflow(conn) = inbox.accept(1, seq, false, b"") {
            ended = Some((conn, seq));
            break;
        }
    }
    assert_eq!(ended, Some((1, CONN_CHUNKS_MAX as u64 + 1)));
}

#[test]
fn the_inbox_never_holds_more_than_its_bound_across_streams() {
    let mut inbox = Inbox::default();
    let chunk = vec![1u8; 60 * 1024];
    for conn in 1..=16u64 {
        // Behind a gap, so none of it can be read.
        for seq in 1..=15u64 {
            let _ = inbox.accept(conn, seq, false, &chunk);
            assert!(inbox.held_bytes() <= HELD_MAX);
        }
    }
    assert!(inbox.held_bytes() > HELD_MAX / 2);
}

#[test]
fn past_the_shared_bound_the_heaviest_stream_is_the_one_ended() {
    let mut inbox = Inbox::default();
    let chunk = vec![1u8; 60 * 1024];
    // One stream piles up near its own bound; the rest hold a little each.
    let mut heavy_seq = 1;
    while (heavy_seq as usize + 1) * chunk.len() <= CONN_HELD_MAX {
        assert_eq!(inbox.accept(9, heavy_seq, false, &chunk), Accept::Held);
        heavy_seq += 1;
    }
    let mut ended = None;
    'fill: for conn in 1..=200u64 {
        for seq in 1..=4u64 {
            if let Accept::Overflow(victim) = inbox.accept(conn, seq, false, &chunk) {
                ended = Some(victim);
                break 'fill;
            }
        }
    }
    assert_eq!(ended, Some(9), "the stream holding the most goes, not the newest");
    assert_eq!(inbox.drain(9, ANSWER_MAX), (Vec::new(), true));
}

#[test]
fn bytes_stuck_behind_a_gap_are_reported_once_the_gap_outlasts_a_resend() {
    let mut inbox = Inbox::default();
    inbox.tick(1_000);
    let _ = inbox.accept(1, 0, false, b"head");
    assert_eq!(inbox.drain(1, ANSWER_MAX).0, b"head");
    let _ = inbox.accept(1, 2, false, b"after the gap");
    inbox.tick(1_000 + GAP_MS);
    assert_eq!(inbox.stalled(1), None, "not yet: the exit may still resend");
    inbox.tick(1_000 + GAP_MS + 1);
    assert_eq!(inbox.stalled(1), Some(1), "message 1 is the one that never came");
}

#[test]
fn a_slow_stream_that_keeps_moving_is_never_reported() {
    let mut inbox = Inbox::default();
    let mut now = 0;
    for seq in 0..20u64 {
        inbox.tick(now);
        // The next one is always already here, behind a gap that fills
        // just inside the window.
        let _ = inbox.accept(1, seq + 1, false, b"next");
        let _ = inbox.accept(1, seq, false, b"this");
        assert!(!inbox.drain(1, 4).0.is_empty());
        now += GAP_MS - 1;
        inbox.tick(now);
        assert_eq!(inbox.stalled(1), None, "at {seq}");
    }
}

#[test]
fn an_idle_stream_with_nothing_held_is_not_stalled() {
    let mut inbox = Inbox::default();
    inbox.tick(0);
    let _ = inbox.accept(1, 0, false, b"all of it");
    let _ = inbox.drain(1, ANSWER_MAX);
    inbox.tick(10 * GAP_MS);
    assert_eq!(inbox.stalled(1), None, "a kept connection waiting for a request is fine");
}
