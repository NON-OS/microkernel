// NONOS Operating System (AGPL-3.0-or-later)
//! Named streams: one caller, several conversations, each its own.
//!
//! A conversation was keyed on the caller's pid alone, so the browser could
//! hold one and fetched a page's resources in turn, a SOCKS handshake each.
//! A frame may now name a stream; the pid the kernel attests stays half of
//! the key, the old frames are stream 0, and one caller is held to a bound.

use std::cell::Cell;

use crate::clients::{Clients, MAX_CLIENTS};
use crate::kept::answer;
use crate::manager::Manager;
use crate::request::{ask, Ask, STREAM_NUMBERED_ON, STREAM_RESET, STREAM_RESET_ON};
use crate::who::{pid_of, who, STREAMS_PER_CALLER, UNNAMED};

fn on(stream: u32, seq: u32, bytes: &[u8]) -> Vec<u8> {
    let mut f = vec![STREAM_NUMBERED_ON];
    f.extend_from_slice(&stream.to_le_bytes());
    f.extend_from_slice(&seq.to_le_bytes());
    f.extend_from_slice(bytes);
    f
}

#[test]
fn a_frame_naming_a_stream_parses_and_the_old_frames_still_do() {
    match ask(&on(3, 7, b"GET /")) {
        Some(Ask::NumberedOn(3, 7, b)) => assert_eq!(b, b"GET /"),
        _ => panic!("a named numbered frame"),
    }
    assert!(matches!(ask(&[STREAM_RESET_ON, 9, 0, 0, 0]), Some(Ask::ResetOn(9))));
    assert!(matches!(ask(&[STREAM_RESET]), Some(Ask::Reset)), "the old reset is stream 0's");
    assert!(ask(&on(0, 1, b"x")).is_none(), "stream 0 is never named");
    assert!(ask(&[STREAM_RESET_ON, 0, 0, 0, 0]).is_none());
    assert!(ask(&[STREAM_NUMBERED_ON, 1, 0, 0, 0, 1, 0, 0]).is_none(), "no number");
    assert!(ask(&[STREAM_RESET_ON, 1, 0]).is_none(), "no stream");
}

#[test]
fn the_pid_is_half_of_the_key_and_no_stream_reaches_anothers() {
    assert_ne!(who(10, 1), who(11, 1));
    assert_ne!(who(10, 1), who(10, 2));
    assert_eq!(who(10, UNNAMED), 10u64 << 32);
    for (pid, stream) in [(1, 0), (u32::MAX, u32::MAX), (77, 3)] {
        assert_eq!(pid_of(who(pid, stream)), pid);
    }
}

#[test]
fn each_stream_keeps_its_own_answer() {
    let pid = 81_001;
    let carried = Cell::new(0);
    let ask_on = |stream: u32, body: &'static [u8], reply: u8| {
        answer(who(pid, stream), 1, body, |_, _| {
            carried.set(carried.get() + 1);
            vec![0, reply]
        })
    };
    assert_eq!(ask_on(1, b"one", b'a'), vec![0, b'a']);
    assert_eq!(ask_on(2, b"two", b'b'), vec![0, b'b']);
    assert_eq!(carried.get(), 2, "the same number on two streams is two exchanges");
    assert_eq!(ask_on(1, b"one", b'x'), vec![0, b'a'], "stream 1 asked again: its own answer");
    assert_eq!(ask_on(2, b"two", b'y'), vec![0, b'b']);
    assert_eq!(carried.get(), 2, "nothing carried twice");
}

#[test]
fn one_caller_holds_a_bounded_share_of_the_table() {
    let mut c = Clients::new();
    for stream in 1..=STREAMS_PER_CALLER as u32 {
        assert!(c.get(who(500, stream)).is_some(), "stream {stream}");
    }
    assert!(c.get(who(500, 99)).is_none(), "past its share, refused");
    assert!(c.get(who(500, 1)).is_some(), "its own streams still answer");
    assert!(c.get(who(501, 1)).is_some(), "another caller is served");
    c.drop_client(who(500, 3));
    assert!(c.get(who(500, 99)).is_some(), "a stream let go frees a place");
    const { assert!(STREAMS_PER_CALLER < MAX_CLIENTS) };
}

#[test]
fn a_caller_naming_ever_more_streams_keeps_a_bounded_number_of_answers() {
    let pid = 82_002;
    let carried_again = |stream: u32| {
        let called = Cell::new(false);
        let _ = answer(who(pid, stream), 1, b"x", |_, _| {
            called.set(true);
            vec![0]
        });
        called.get()
    };
    let streams = STREAMS_PER_CALLER as u32 + 3;
    for stream in 1..=streams {
        assert!(carried_again(stream));
    }
    assert!(!carried_again(streams), "the newest is kept");
    assert!(carried_again(1), "the oldest went to keep the bound");
}

#[test]
fn every_stream_of_a_caller_gets_a_tunnel_of_its_own() {
    let mut m = Manager::new();
    let a = m.open(who(600, 1)).expect("first");
    let b = m.open(who(600, 2)).expect("second");
    assert_ne!(a, b);
    assert_eq!(m.id_of_socket(who(600, 2)), Some(b));
    assert_eq!(m.close_socket(who(600, 1)), Some(a));
    assert_eq!(m.id_of_socket(who(600, 2)), Some(b), "closing one leaves the other");
}
