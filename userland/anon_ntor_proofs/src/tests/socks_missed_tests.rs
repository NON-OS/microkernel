// NONOS Operating System (AGPL-3.0-or-later)
/*
 * New bytes under the number of an answer the caller never received. The
 * caller moves to its next number only once an answer arrives, so its next
 * write after a lost one goes under the old number: the browser's TLS hello
 * or request, after a 60 ms poll it stopped waiting for. The front gave the
 * kept answer back for it and dropped the bytes while the caller counted
 * them sent. It now carries them, with the missed answer in front, as
 * net.socks5 does.
 */

use super::socks::front::Front;
use super::socks::stage::OUT_MAX;
use super::socks::tunnel::Far;
use super::socks_fake::{fake, Fake};
use super::socks_frames::{connect_example, numbered, HELLO, PID};

/// A conversation open on a stream, its handshake under numbers 1 and 2.
fn open() -> (Front, Fake) {
    let (mut front, mut t) = (Front::default(), fake());
    t.far = Far::Open;
    assert_eq!(front.serve(&mut t, PID, &numbered(1, &HELLO)), [0, 5, 0]);
    let reply = front.serve(&mut t, PID, &numbered(2, &connect_example()));
    assert_eq!(reply[..3], [0, 5, 0]);
    (front, t)
}

#[test]
fn new_bytes_under_a_number_whose_answer_was_lost_are_carried() {
    let (mut front, mut t) = open();
    t.arrived.extend_from_slice(b"early");
    /* A poll under 3; its answer, carrying "early", never reaches the caller. */
    let lost = front.serve(&mut t, PID, &numbered(3, &[]));
    assert_eq!(&lost[..], b"\0early");
    t.arrived.extend_from_slice(b" late");
    /* The caller's next write goes under 3 again, with new bytes. */
    let answer = front.serve(&mut t, PID, &numbered(3, b"client hello"));
    assert_eq!(t.sent, b"client hello", "the new bytes reached the stream");
    assert_eq!(&answer[..], b"\0early late", "the missed answer first, then what came since");
    /* Asked again unchanged, it is the same answer and nothing goes twice. */
    assert_eq!(front.serve(&mut t, PID, &numbered(3, b"client hello")), answer);
    assert_eq!(t.sent, b"client hello");
}

#[test]
fn the_same_bytes_asked_again_are_carried_once() {
    let (mut front, mut t) = open();
    let first = front.serve(&mut t, PID, &numbered(3, b"request"));
    t.arrived.extend_from_slice(b"reply");
    assert_eq!(front.serve(&mut t, PID, &numbered(3, b"request")), first);
    assert_eq!(t.sent, b"request", "carried once");
    assert_eq!(t.arrived, b"reply", "the repeat took nothing off the stream");
}

#[test]
fn a_missed_close_is_the_answer_to_new_bytes() {
    let (mut front, mut t) = open();
    t.arrived.extend_from_slice(b"bye");
    t.far = Far::Ended(6);
    let close = front.serve(&mut t, PID, &numbered(3, &[]));
    assert_eq!(&close[..], b"\x01bye");
    let again = front.serve(&mut t, PID, &numbered(3, b"more"));
    assert_eq!(again, close, "the stream ended; the close is the answer");
    assert!(t.sent.is_empty(), "nothing is carried into an ended stream");
}

#[test]
fn a_missed_answer_and_the_new_one_fit_one_answer() {
    let (mut front, mut t) = open();
    t.arrived = alloc::vec![0x5A; OUT_MAX - 10];
    let lost = front.serve(&mut t, PID, &numbered(3, &[]));
    assert_eq!(lost.len(), 1 + OUT_MAX - 10);
    t.arrived = alloc::vec![0x6B; 100];
    let answer = front.serve(&mut t, PID, &numbered(3, b"x"));
    assert_eq!(answer.len(), 1 + OUT_MAX, "one answer, as large as one can be");
    assert_eq!(answer[1..1 + OUT_MAX - 10], lost[1..], "the missed bytes, whole and first");
    assert_eq!(t.arrived.len(), 90, "what did not fit waits for the next exchange");
    let next = front.serve(&mut t, PID, &numbered(4, &[]));
    assert_eq!(next.len(), 1 + 90);
}
