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


//! Reading a mirror's reply through the network the person chose: the
//! reader waits long for the first byte and per quiet stretch after it,
//! stops once the reply is whole or the far end has finished, keeps what
//! came before a break, and refuses a reply larger than anything it fetches.
//! The stream is scripted here; the capsule reads a `RouteStream`.

use alloc::vec::Vec;

use crate::install::http_reply::{body, complete};
use crate::install::route_read::{offline, read_reply, Reply, Source, CHUNK, FIRST_MS, QUIET_MS};

/// A stream that hands out `pieces` in order, then zeros, and records the
/// wait each read was given.
struct Script {
    pieces: Vec<Result<Vec<u8>, &'static str>>,
    next: usize,
    ends_after_last: bool,
    waits: Vec<u64>,
}

impl Script {
    fn new(pieces: Vec<Result<Vec<u8>, &'static str>>) -> Script {
        Script { pieces, next: 0, ends_after_last: false, waits: Vec::new() }
    }
}

impl Source for Script {
    fn read_wait(&mut self, into: &mut [u8], wait_ms: u64) -> Result<usize, &'static str> {
        self.waits.push(wait_ms);
        let Some(piece) = self.pieces.get(self.next) else { return Ok(0) };
        self.next += 1;
        let bytes = piece.clone()?;
        assert!(bytes.len() <= into.len(), "a read asks for at most CHUNK bytes");
        into[..bytes.len()].copy_from_slice(&bytes);
        Ok(bytes.len())
    }

    fn ended(&self) -> bool {
        self.ends_after_last && self.next >= self.pieces.len()
    }
}

const REPLY: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Length: 11\r\n\r\nhello world";

fn split(at: &[usize]) -> Vec<Result<Vec<u8>, &'static str>> {
    let mut out = Vec::new();
    let mut from = 0;
    for &to in at.iter().chain(core::iter::once(&REPLY.len())) {
        out.push(Ok(REPLY[from..to].to_vec()));
        from = to;
    }
    out
}

#[test]
fn a_reply_in_pieces_is_read_until_it_is_whole() {
    let mut s = Script::new(split(&[5, 20, 40]));
    /* A piece the mirror would send after the reply is never asked for. */
    s.pieces.push(Ok(b"trailing".to_vec()));
    let got = read_reply(&mut s, 1 << 20, &complete);
    assert_eq!(got, Reply::Got(REPLY.to_vec()));
    assert_eq!(s.next, 4, "reading stopped once the reply was whole");
    let Reply::Got(raw) = got else { unreachable!() };
    assert_eq!(body(raw).as_deref(), Some(&b"hello world"[..]));
}

#[test]
fn the_first_byte_gets_the_long_wait_and_each_piece_after_the_quiet_one() {
    let mut s = Script::new(split(&[5, 20]));
    let _ = read_reply(&mut s, 1 << 20, &complete);
    assert_eq!(s.waits, [FIRST_MS, QUIET_MS, QUIET_MS]);
}

#[test]
fn a_far_end_that_finishes_stops_the_read_without_another_wait() {
    /* No length: only the far end's finishing says it is over. */
    let mut s = Script::new(vec![Ok(b"HTTP/1.1 200 OK\r\n\r\nall of it".to_vec())]);
    s.ends_after_last = true;
    let got = read_reply(&mut s, 1 << 20, &complete);
    assert_eq!(got, Reply::Got(b"HTTP/1.1 200 OK\r\n\r\nall of it".to_vec()));
    assert_eq!(s.waits.len(), 1);
}

#[test]
fn nothing_at_all_is_nothing() {
    let mut s = Script::new(Vec::new());
    assert_eq!(read_reply(&mut s, 1 << 20, &complete), Reply::Nothing);
    assert_eq!(s.waits, [FIRST_MS], "one long wait for a first byte, then it gives up");
}

#[test]
fn a_break_before_anything_is_nothing_and_after_keeps_what_came() {
    let mut s = Script::new(vec![Err("broken")]);
    assert_eq!(read_reply(&mut s, 1 << 20, &complete), Reply::Nothing);

    let mut s = Script::new(vec![Ok(REPLY[..20].to_vec()), Err("broken")]);
    let got = read_reply(&mut s, 1 << 20, &complete);
    assert_eq!(got, Reply::Got(REPLY[..20].to_vec()));
    /* What came is judged by its framing: a cut reply is no body. */
    let Reply::Got(raw) = got else { unreachable!() };
    assert_eq!(body(raw), None);
}

#[test]
fn a_reply_that_stops_short_of_its_length_is_no_body() {
    let mut s = Script::new(split(&[30]));
    s.pieces.pop();
    let Reply::Got(raw) = read_reply(&mut s, 1 << 20, &complete) else { panic!("something came") };
    assert!(!complete(&raw));
    assert_eq!(body(raw), None);
}

#[test]
fn more_than_the_limit_is_refused_whole() {
    let big = vec![b'x'; 100];
    let mut s = Script::new(vec![Ok(big.clone()), Ok(big)]);
    assert_eq!(read_reply(&mut s, 150, &|_| false), Reply::TooLarge);
}

#[test]
fn a_read_asks_for_a_bounded_chunk() {
    let mut s = Script::new(vec![Ok(vec![0u8; CHUNK])]);
    s.ends_after_last = true;
    assert_eq!(read_reply(&mut s, usize::MAX, &|_| false), Reply::Got(vec![0u8; CHUNK]));
}

#[test]
fn an_install_waits_for_no_network_to_reach_a_local_mirror() {
    assert_eq!(offline(true, || None), None);
    assert_eq!(offline(true, || Some("Nym is not running")), None);
    /* The network is not even asked: asking waits up to three minutes. */
    assert_eq!(offline(true, || panic!("a local mirror waited for the network")), None);
}

#[test]
fn any_other_mirror_needs_the_chosen_network_running() {
    assert_eq!(offline(false, || None), None);
    assert_eq!(offline(false, || Some("Nym is not running")), Some("Nym is not running"));
}
