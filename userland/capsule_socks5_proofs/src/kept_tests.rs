// NONOS Operating System (AGPL-3.0-or-later)
//! Numbered exchanges: an answer whose reply was lost is given again.
//!
//! The server takes exit bytes out of its inbox to build an answer; the
//! kernel drops a reply that arrives after its caller stopped waiting. These
//! tests drive the server's own request parser and kept store the way its
//! run loop does, with a stand-in for the inbox.

use crate::kept_harness::{numbered, serve, Exit};
use crate::reply::{ANSWER_MAX, STREAM_CLOSED, STREAM_OPEN};
use crate::request::{ask, Ask, STREAM_NUMBERED};

fn open(bytes: &[u8]) -> Vec<u8> {
    let mut out = vec![STREAM_OPEN];
    out.extend_from_slice(bytes);
    out
}

#[test]
fn a_numbered_frame_parses_and_a_short_one_does_not() {
    match ask(&numbered(7, b"GET /")) {
        Some(Ask::Numbered(7, b)) => assert_eq!(b, b"GET /"),
        _ => panic!("numbered frame not recognised"),
    }
    assert!(ask(&[STREAM_NUMBERED, 1, 2]).is_none());
}

#[test]
fn a_lost_answer_is_given_again_and_the_next_number_reads_on() {
    let pid = 9001;
    let mut exit = Exit::with(&[b"first", b"second"]);
    /* Exchange 1 is answered but the reply never reaches the caller. */
    assert_eq!(serve(pid, &numbered(1, b""), &mut exit), open(b"first"));
    /* The caller asks exchange 1 again and gets the same bytes, not nothing. */
    assert_eq!(serve(pid, &numbered(1, b""), &mut exit), open(b"first"));
    /* Having them, it moves to exchange 2, which reads on. */
    assert_eq!(serve(pid, &numbered(2, b""), &mut exit), open(b"second"));
}

#[test]
fn unnumbered_frames_read_as_before() {
    let pid = 9002;
    let mut exit = Exit::with(&[b"a", b"b"]);
    assert_eq!(serve(pid, &[0], &mut exit), open(b"a"));
    assert_eq!(serve(pid, &[0], &mut exit), open(b"b"));
}

#[test]
fn a_reset_forgets_the_kept_answer() {
    let pid = 9003;
    let mut exit = Exit::with(&[b"old", b"new"]);
    assert_eq!(serve(pid, &numbered(4, b""), &mut exit), open(b"old"));
    serve(pid, &[1], &mut exit);
    assert_eq!(serve(pid, &numbered(4, b""), &mut exit), open(b"new"), "a fresh conversation");
}

#[test]
fn bytes_sent_under_a_lost_polls_number_are_carried() {
    /* The browser polls under 7 after its CONNECT and stops waiting; its
     * next write, the TLS hello, goes out under 7 as well. */
    let pid = 9004;
    let mut exit = Exit::with(&[b"", b"server hello"]);
    serve(pid, &numbered(7, b""), &mut exit);
    let answer = serve(pid, &numbered(7, b"client hello"), &mut exit);
    assert_eq!(exit.carried, vec![b"client hello".to_vec()], "the hello reached the exit");
    assert_eq!(answer, open(b"server hello"));
}

#[test]
fn the_answer_a_caller_missed_goes_in_front_of_what_new_bytes_bring() {
    let pid = 9005;
    let mut exit = Exit::with(&[b"early ", b"reply"]);
    serve(pid, &numbered(3, b""), &mut exit);
    assert_eq!(serve(pid, &numbered(3, b"hello"), &mut exit), open(b"early reply"));
    /* That answer is lost too: a poll under 3 gets all of it, carrying nothing. */
    assert_eq!(serve(pid, &numbered(3, b""), &mut exit), open(b"early reply"));
    assert_eq!(exit.carried, vec![b"hello".to_vec()]);
}

#[test]
fn the_same_bytes_asked_again_are_carried_once() {
    let pid = 9006;
    let mut exit = Exit::with(&[b"ack", b"more"]);
    assert_eq!(serve(pid, &numbered(5, b"record"), &mut exit), open(b"ack"));
    assert_eq!(serve(pid, &numbered(5, b"record"), &mut exit), open(b"ack"));
    assert_eq!(exit.carried, vec![b"record".to_vec()], "a retransmit is not a second write");
}

#[test]
fn a_close_the_caller_missed_answers_new_bytes() {
    let pid = 9007;
    let mut exit = Exit::with(&[b"bye"]);
    exit.closed = true;
    let close = serve(pid, &numbered(2, b""), &mut exit);
    assert_eq!(close[0], STREAM_CLOSED);
    assert_eq!(serve(pid, &numbered(2, b"late write"), &mut exit), close);
    assert!(exit.carried.is_empty(), "nothing is carried to a finished stream");
}

#[test]
fn a_joined_answer_stays_within_the_bound() {
    let pid = 9008;
    let missed = vec![b'm'; ANSWER_MAX - 10];
    let mut exit = Exit::with(&[&missed, &[b'n'; 50]]);
    serve(pid, &numbered(9, b""), &mut exit);
    let joined = serve(pid, &numbered(9, b"data"), &mut exit);
    assert_eq!(joined.len(), 1 + ANSWER_MAX, "missed bytes and as many new ones as fit");
    assert_eq!(&joined[1 + missed.len()..], &[b'n'; 10]);
    assert_eq!(exit.answers.front().map(Vec::len), Some(40), "the rest waits for the next answer");
}
