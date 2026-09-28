// NONOS Operating System (AGPL-3.0-or-later)
//! Numbered exchanges: an answer whose reply was lost is given again.
//!
//! The server takes exit bytes out of its inbox to build an answer; the
//! kernel drops a reply that arrives after its caller stopped waiting. These
//! tests drive the server's own request parser and kept store the way its
//! run loop does, with a stand-in for the inbox.

use std::cell::RefCell;
use std::collections::VecDeque;

use crate::kept::{again, forget, keep};
use crate::request::{ask, Ask, STREAM_NUMBERED};

fn numbered(seq: u32, payload: &[u8]) -> Vec<u8> {
    let mut f = vec![STREAM_NUMBERED];
    f.extend_from_slice(&seq.to_le_bytes());
    f.extend_from_slice(payload);
    f
}

/// What run.rs does for one request from `pid`, with `drain` standing in
/// for feed(): each call takes whatever the exit has delivered so far.
fn serve(pid: u32, frame: &[u8], drain: &mut dyn FnMut() -> Vec<u8>) -> Vec<u8> {
    match ask(frame) {
        Some(Ask::Numbered(seq, _)) => match again(pid, seq) {
            Some(out) => out,
            None => {
                let out = drain();
                keep(pid, seq, &out);
                out
            }
        },
        Some(Ask::Stream(_)) => drain(),
        _ => {
            forget(pid);
            Vec::new()
        }
    }
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
    let exit: RefCell<VecDeque<Vec<u8>>> =
        RefCell::new(VecDeque::from(vec![b"first".to_vec(), b"second".to_vec()]));
    let mut drain = || exit.borrow_mut().pop_front().unwrap_or_default();
    // Exchange 1 is answered but the reply never reaches the caller.
    let lost = serve(pid, &numbered(1, b""), &mut drain);
    assert_eq!(lost, b"first");
    // The caller asks exchange 1 again and gets the same bytes, not nothing.
    assert_eq!(serve(pid, &numbered(1, b""), &mut drain), b"first");
    // Having them, it moves to exchange 2, which reads on.
    assert_eq!(serve(pid, &numbered(2, b""), &mut drain), b"second");
    forget(pid);
}

#[test]
fn unnumbered_frames_read_as_before() {
    let pid = 9002;
    let exit: RefCell<VecDeque<Vec<u8>>> =
        RefCell::new(VecDeque::from(vec![b"a".to_vec(), b"b".to_vec()]));
    let mut drain = || exit.borrow_mut().pop_front().unwrap_or_default();
    assert_eq!(serve(pid, &[0], &mut drain), b"a");
    assert_eq!(serve(pid, &[0], &mut drain), b"b");
}

#[test]
fn a_reset_forgets_the_kept_answer() {
    let pid = 9003;
    keep(pid, 4, b"old");
    assert_eq!(again(pid, 4).as_deref(), Some(&b"old"[..]));
    let mut nothing = Vec::new;
    serve(pid, &[1], &mut nothing);
    assert_eq!(again(pid, 4), None);
}
