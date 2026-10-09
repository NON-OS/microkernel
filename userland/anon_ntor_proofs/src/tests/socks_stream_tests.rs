// NONOS Operating System (AGPL-3.0-or-later)
/* Named streams: one caller, several conversations through net.anon, each
 * its own handshake, its own Anyone stream and its own kept answer; the
 * old frames are stream 0; one caller is held to its share; and a caller
 * that ended has its conversations ended for it. */

use alloc::vec::Vec;

use super::socks::front::{Front, CALLERS_MAX};
use super::socks::tunnel::Far;
use super::socks_fake::fake;
use super::socks_frames::{bytes, connect_example, numbered, HELLO, PID};

const STREAMS_PER_CALLER: usize = 8;

fn on(stream: u32, seq: u32, frame: &[u8]) -> Vec<u8> {
    [&[4u8][..], &stream.to_le_bytes(), &seq.to_le_bytes(), frame].concat()
}

fn reset_on(stream: u32) -> Vec<u8> {
    [&[3u8][..], &stream.to_le_bytes()].concat()
}

#[test]
fn two_named_streams_of_one_caller_are_two_conversations() {
    let (mut front, mut t) = (Front::default(), fake());
    assert_eq!(front.serve(&mut t, PID, &on(1, 1, &HELLO)), [0, 5, 0]);
    assert_eq!(front.serve(&mut t, PID, &on(2, 1, &HELLO)), [0, 5, 0], "a greeting, not stream 1's request");
    t.far = Far::Open;
    let ok = [0, 5, 0, 0, 1, 0, 0, 0, 0, 0, 0];
    assert_eq!(front.serve(&mut t, PID, &on(1, 2, &connect_example())), ok);
    assert_eq!(front.serve(&mut t, PID, &on(2, 2, &connect_example())), ok);
    assert_eq!(t.opened.len(), 2, "an Anyone stream each");
    assert_eq!(front.serve(&mut t, PID, &bytes(&HELLO)), [0, 5, 0], "stream 0 is its own too");
}

#[test]
fn a_named_reset_ends_that_stream_only() {
    let (mut front, mut t) = (Front::default(), fake());
    t.far = Far::Open;
    for s in [1, 2] {
        front.serve(&mut t, PID, &on(s, 1, &HELLO));
        front.serve(&mut t, PID, &on(s, 2, &connect_example()));
    }
    assert_eq!(front.serve(&mut t, PID, &reset_on(1)), [0]);
    assert_eq!(t.closed.len(), 1, "stream 1's Anyone stream ended");
    assert_eq!(front.serve(&mut t, PID, &on(2, 3, b"GET")), [0], "stream 2 relays on");
    assert_eq!(t.sent, b"GET");
    assert_eq!(front.serve(&mut t, PID, &on(1, 1, &HELLO)), [0, 5, 0], "stream 1 starts over");
}

#[test]
fn the_same_number_on_two_streams_is_two_exchanges() {
    let (mut front, mut t) = (Front::default(), fake());
    assert_eq!(front.serve(&mut t, PID, &on(1, 1, &HELLO)), [0, 5, 0]);
    assert_eq!(front.serve(&mut t, PID, &on(2, 1, &HELLO)), [0, 5, 0]);
    assert_eq!(front.serve(&mut t, PID, &on(1, 1, &HELLO)), [0, 5, 0], "kept, not carried again");
    assert_eq!(front.serve(&mut t, PID, &numbered(1, &HELLO)), [0, 5, 0], "stream 0 apart");
}

#[test]
fn one_caller_is_held_to_its_share_and_others_are_served() {
    let (mut front, mut t) = (Front::default(), fake());
    for s in 1..=STREAMS_PER_CALLER as u32 {
        assert_eq!(front.serve(&mut t, PID, &on(s, 1, &[5])), [0], "stream {s}");
    }
    assert_eq!(front.serve(&mut t, PID, &on(99, 1, &HELLO)), [1, 5, 0xFF], "past its share");
    assert_eq!(front.serve(&mut t, PID + 1, &on(1, 1, &HELLO)), [0, 5, 0], "another caller");
    const { assert!(STREAMS_PER_CALLER < CALLERS_MAX) };
}

#[test]
fn a_frame_naming_stream_zero_is_no_frame() {
    let (mut front, mut t) = (Front::default(), fake());
    assert_eq!(front.serve(&mut t, PID, &on(0, 1, &HELLO)), [1], "refused and closed");
    assert_eq!(front.serve(&mut t, PID, &[4, 1, 0, 0, 0, 1]), [1], "too short");
}

#[test]
fn an_ended_callers_conversations_are_ended_for_it() {
    let (mut front, mut t) = (Front::default(), fake());
    t.far = Far::Open;
    for (pid, s) in [(PID, 1), (PID, 2), (PID + 1, 1)] {
        front.serve(&mut t, pid, &on(s, 1, &HELLO));
        front.serve(&mut t, pid, &on(s, 2, &connect_example()));
    }
    front.reap(&mut t, |pid| pid != PID);
    assert_eq!(t.closed.len(), 2, "the two of the caller that ended");
    assert_eq!(front.serve(&mut t, PID + 1, &on(1, 3, b"x")), [0], "the living caller relays on");
    assert_eq!(front.serve(&mut t, PID, &on(1, 2, &[])), [2], "its kept answer is gone too: lost");
}
