// NONOS Operating System (AGPL-3.0-or-later)
//! What net.socks5 says before net.nym is up, how it answers a status ask,
//! and when it says a conversation was lost. Before, a parked proxy closed
//! every frame and a restarted one read a caller's stream bytes as a SOCKS
//! greeting: either way the caller could only blame the exit.

use crate::parked::parked;
use crate::reply::{progress, STATUS, STATUS_VERSION, STEPS, STREAM_CLOSED, STREAM_LOST, STREAM_OPEN};
use crate::request::{lost, FIRST_SEQ, STATUS_ASK, STREAM_NUMBERED_ON, STREAM_RESET_ON};
use crate::wire::REP_NET_UNREACH;

fn on(stream: u32, seq: u32, body: &[u8]) -> Vec<u8> {
    let mut f = vec![STREAM_NUMBERED_ON];
    f.extend_from_slice(&stream.to_le_bytes());
    f.extend_from_slice(&seq.to_le_bytes());
    f.extend_from_slice(body);
    f
}

#[test]
fn a_parked_proxy_says_it_waits_for_net_nym() {
    assert_eq!(parked(&[STATUS_ASK]), [STATUS, STATUS_VERSION, 0, 1, STEPS, 0]);
}

#[test]
fn a_parked_proxy_takes_the_greeting_and_refuses_the_connect_as_not_yet() {
    assert_eq!(parked(&on(3, 1, &[5, 1, 0])), [STREAM_OPEN, 5, 0]);
    let connect = [5, 1, 0, 3, 11, b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.', b'o', b'r', b'g', 0, 80];
    let got = parked(&on(3, 2, &connect));
    assert_eq!(got[0], STREAM_CLOSED);
    assert_eq!(&got[1..3], &[5, REP_NET_UNREACH], "3 is what every client reads as not yet");
    assert_eq!(got.len(), 11);
}

#[test]
fn a_parked_proxy_answers_the_same_frame_the_same_way() {
    let f = on(2, 1, &[5, 1, 0]);
    assert_eq!(parked(&f), parked(&f), "a frame asked again is answered again");
}

#[test]
fn a_parked_proxy_answers_polls_resets_and_rubbish() {
    assert_eq!(parked(&on(1, 1, &[])), [STREAM_OPEN]);
    let mut reset = vec![STREAM_RESET_ON];
    reset.extend_from_slice(&4u32.to_le_bytes());
    assert_eq!(parked(&reset), [STREAM_OPEN]);
    assert_eq!(parked(&[9, 9, 9]), [STREAM_CLOSED]);
    assert_eq!(parked(&on(1, 1, &[0x16, 3, 1, 0, 5])), [STREAM_CLOSED]);
    assert_eq!(parked(&on(1, 1, &[5, 1, 2])), [STREAM_CLOSED, 5, 0xFF], "no acceptable method");
}

#[test]
fn a_ready_proxy_says_so() {
    assert_eq!(progress(true, 2, 0), [STATUS, STATUS_VERSION, 1, 2, STEPS, 0]);
    assert_eq!(progress(false, 3, 4), [STATUS, STATUS_VERSION, 0, 3, STEPS, 4], "trying another exit, four walked away from");
}

#[test]
fn only_a_later_exchange_of_an_unknown_conversation_is_lost() {
    assert!(lost(FIRST_SEQ + 1, false, false), "restarted under the caller");
    assert!(!lost(FIRST_SEQ, false, false), "a new conversation starts at the first number");
    assert!(!lost(9, true, false), "a held conversation carries on");
    assert!(!lost(9, false, true), "a kept answer is given back, a close among them");
    assert_ne!(STREAM_LOST, STREAM_OPEN);
    assert_ne!(STREAM_LOST, STREAM_CLOSED);
}
