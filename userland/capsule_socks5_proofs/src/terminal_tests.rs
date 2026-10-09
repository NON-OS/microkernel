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

//! Proofs that the terminal speaks this proxy's framing: what it sends is
//! read as numbered stream bytes, what comes back is read past its marker
//! with a close noticed, a missed answer is asked for again under the same
//! number and given back without the bytes being carried twice; and that
//! the terminal leaves only by the network the person chose, never directly
//! unless Direct is that choice, an unreadable policy store included.
//!
//! The terminal's client is nonos_route_link's now, so these are its frames
//! and its rule, held against this proxy's own parsing.

use nonos_policy_proto::route;

use crate::kept_harness::{serve, Exit};
use crate::reply::Reply;
use crate::request::{ask, Ask};
use crate::route_answer::{decode, Answer, Malformed};
use crate::route_frame::{next_seq, numbered, reset, FIRST_SEQ};
use crate::route_pick::{pick, Route, ANYONE_DOWN, NYM_DOWN, UNREAD};

const SOCKS: u32 = 4908;
const ANON: u32 = 4911;

fn frame(seq: u32, bytes: &[u8]) -> Vec<u8> {
    numbered(seq, bytes).unwrap_or_default()
}

/// The terminal used to send SOCKS bytes bare. The proxy reads the first
/// byte of every request as a marker, and the greeting's version byte, 5,
/// is no marker it knows: it forgot the conversation and answered with an
/// empty open marker, so the terminal never got past its greeting.
#[test]
fn a_bare_greeting_is_not_a_request_the_proxy_reads() {
    assert!(ask(&[5, 1, 0]).is_none());
}

#[test]
fn the_terminal_s_frames_are_read_as_numbered_stream_bytes() {
    match ask(&frame(FIRST_SEQ, &[5, 1, 0])) {
        Some(Ask::Numbered(1, body)) => assert_eq!(body, [5, 1, 0]),
        _ => panic!("a greeting under exchange 1"),
    }
    match ask(&frame(FIRST_SEQ, &[])) {
        Some(Ask::Numbered(1, body)) => assert!(body.is_empty(), "a poll carries nothing"),
        _ => panic!("a poll under the same number"),
    }
    assert!(matches!(ask(&reset()), Some(Ask::Reset)));
}

#[test]
fn the_terminal_reads_past_the_marker_and_notices_a_close() {
    let open = Reply::open(b"\x05\x00".to_vec()).encode();
    assert_eq!(decode(&open), Ok(Answer { closed: false, bytes: &[5, 0] }));
    let closed = Reply::closed(b"last".to_vec()).encode();
    assert_eq!(decode(&closed), Ok(Answer { closed: true, bytes: b"last" }));
    assert_eq!(decode(&[]), Err(Malformed::Empty), "an empty answer is not one the proxy sends");
}

/// A call that timed out leaves the number where it was, so the same bytes
/// go again as the same frame: the proxy carries them once and answers the
/// repeat with what it kept.
#[test]
fn a_write_whose_answer_was_lost_is_carried_once() {
    let mut exit = Exit::with(&[b"server hello"]);
    let first = serve(901, &frame(FIRST_SEQ, b"client hello"), &mut exit);
    // The answer never reached the terminal: it asks again, unchanged.
    let again = serve(901, &frame(FIRST_SEQ, b"client hello"), &mut exit);
    assert_eq!(first, again, "the same answer, given again");
    assert_eq!(exit.carried, vec![b"client hello".to_vec()], "the bytes went once");
    assert_eq!(decode(&again).map(|a| a.bytes.to_vec()), Ok(b"server hello".to_vec()));
    // The next exchange is a new number and gets new bytes.
    match ask(&frame(next_seq(FIRST_SEQ), &[])) {
        Some(Ask::Numbered(2, _)) => {}
        _ => panic!("the number moves on once an answer is taken"),
    }
}

#[test]
fn a_poll_whose_answer_was_lost_collects_it_next_time() {
    let mut exit = Exit::with(&[b"late bytes", b"after"]);
    let _lost = serve(902, &frame(FIRST_SEQ, &[]), &mut exit);
    let kept = serve(902, &frame(FIRST_SEQ, &[]), &mut exit);
    let late = decode(&kept).map(|a| a.bytes.to_vec());
    assert_eq!(late, Ok(b"late bytes".to_vec()), "nothing was lost between the polls");
    let next = serve(902, &frame(next_seq(FIRST_SEQ), &[]), &mut exit);
    assert_eq!(decode(&next).map(|a| a.bytes.to_vec()), Ok(b"after".to_vec()));
}

/// The rule the terminal leaves by. It used to be the proxy's presence:
/// with net.socks5 up everything went through Nym whatever was chosen, and
/// without it a store that could not be asked let the request go directly
/// (`refusal(None) == None`, "the old behaviour"), naming this machine on
/// exactly the machines whose default, the mixnet, says not to. That
/// assertion encoded the leak; the rule now fails closed.
#[test]
fn an_unreadable_store_is_the_mixnet_and_never_direct() {
    assert_eq!(pick(None, SOCKS, ANON), Route::Nym(SOCKS));
    assert_eq!(pick(None, 0, ANON), Route::Down(UNREAD), "no proxy: no route, not direct");
    assert_eq!(pick(None, 0, 0), Route::Down(UNREAD));
}

#[test]
fn the_terminal_leaves_by_the_chosen_network_whichever_proxies_run() {
    for (nym, anon) in [(0, 0), (SOCKS, 0), (0, ANON), (SOCKS, ANON)] {
        /* Direct only when chosen, even with net.socks5 running. */
        assert_eq!(pick(Some(route::DIRECT), nym, anon), Route::Direct);
        /* The Anyone network through net.anon, never through Nym. */
        let anyone = pick(Some(route::ANYONE), nym, anon);
        let want = if anon != 0 { Route::Anon(ANON) } else { Route::Down(ANYONE_DOWN) };
        assert_eq!(anyone, want);
        /* The mixnet, or no route; a value this build does not know too. */
        for default in [Some(route::NYM), Some(200)] {
            let want = if nym != 0 { Route::Nym(SOCKS) } else { Route::Down(NYM_DOWN) };
            assert_eq!(pick(default, nym, anon), want);
        }
    }
}
