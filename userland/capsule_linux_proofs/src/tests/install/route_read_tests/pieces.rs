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

//! A reply read until whole, with its waits, its finish and its breaks.

use alloc::vec::Vec;

use super::script::{split, Script, REPLY};
use crate::install::http_reply::{body, complete};
use crate::install::route_read::read::{FIRST_MS, QUIET_MS};
use crate::install::route_read::{read_reply, Reply};

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
