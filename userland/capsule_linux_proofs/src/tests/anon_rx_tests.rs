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

//! A net.anon stream's receive buffer. net.anon cannot be asked whether
//! bytes wait without taking them, so poll and read take them into the
//! socket's buffer and a read drains it. Held here: the buffer never holds
//! more than one reply, bytes are never put behind bytes or after the end,
//! poll never calls a stream readable on an empty buffer while it is open,
//! and a read reports the far end the way Linux reports a TCP peer.

use super::random::Regs;
use crate::linux::abi::errno::{EAGAIN, ECONNREFUSED, ECONNRESET, EIO, ENETUNREACH, ETIMEDOUT};
use crate::linux::net::anon_ops::{PAYLOAD_MAX, REASON_DESTROY, REASON_DONE, REASON_TIMEOUT};
use crate::linux::net::sock::{Anon, End, Got};

const POLLIN: u16 = 0x001;
const POLLOUT: u16 = 0x004;
const POLLERR: u16 = 0x008;
const POLLHUP: u16 = 0x010;
const POLLRDHUP: u16 = 0x2000;

/// tor-spec 6.3: the exit or the host refused, the exit's policy refused.
const REASON_CONNECTREFUSED: u8 = 3;
const REASON_EXITPOLICY: u8 = 4;

fn open() -> Anon {
    Anon::new(4484, 1)
}

#[test]
fn an_open_stream_with_nothing_held_is_not_readable() {
    let a = open();
    assert_eq!(a.bits(false, false) & POLLIN, 0);
    assert_eq!(a.bits(false, false), POLLOUT, "writable: a write goes, waits or fails at once");
    let mut a = open();
    a.fill(Got::Nothing);
    assert_eq!(a.bits(false, false) & POLLIN, 0, "net.anon had nothing: still not readable");
    assert_eq!(a.take(10, false, false), Err(EAGAIN), "a read waits");
}

#[test]
fn held_bytes_are_readable_and_a_read_drains_them() {
    let mut a = open();
    a.fill(Got::Bytes(b"HTTP/1.1 200 OK".to_vec()));
    assert_eq!(a.bits(false, false) & (POLLIN | POLLHUP | POLLERR), POLLIN);
    assert_eq!(a.take(4, true, false), Ok(b"HTTP".to_vec()), "a peek leaves them");
    assert_eq!(a.take(4, false, false), Ok(b"HTTP".to_vec()));
    assert_eq!(a.take(100, false, false), Ok(b"/1.1 200 OK".to_vec()));
    assert_eq!(a.bits(false, false) & POLLIN, 0, "drained, and still open");
    assert_eq!(a.take(1, false, false), Err(EAGAIN));
}

#[test]
fn nothing_is_put_behind_held_bytes_or_after_the_end() {
    let mut a = open();
    a.fill(Got::Bytes(vec![1; 10]));
    assert!(!a.wants_fill(), "a full buffer asks net.anon for nothing");
    a.fill(Got::Bytes(vec![2; 10]));
    a.fill(Got::End(REASON_DONE));
    assert_eq!(a.rx, vec![1; 10]);
    assert_eq!(a.end, None);
    assert_eq!(a.take(10, false, false), Ok(vec![1; 10]));
    a.fill(Got::End(REASON_DONE));
    a.fill(Got::Bytes(vec![3; 10]));
    assert!(a.rx.is_empty(), "nothing after the end");
    assert_eq!(a.take(10, false, false), Ok(Vec::new()), "end of file");
}

#[test]
fn the_buffer_never_holds_more_than_one_reply() {
    let mut r = Regs::new(0x5eed_0a11);
    for _ in 0..2000 {
        let mut a = open();
        for _ in 0..12 {
            let len = match r.small(4) {
                0 => PAYLOAD_MAX,
                1 => PAYLOAD_MAX + 1 + r.small(64) as usize,
                _ => r.small(PAYLOAD_MAX as u64 + 1) as usize,
            };
            match r.small(5) {
                0 => a.fill(Got::Bytes(vec![0x55; len])),
                1 => a.fill(Got::Nothing),
                2 => {
                    let _ =
                        a.take(r.small(PAYLOAD_MAX as u64 * 2) as usize, r.small(2) == 0, false);
                }
                3 => a.fill(Got::End(r.small(256) as u8)),
                _ => {
                    let _ = a.take(len, false, false);
                }
            }
            assert!(a.rx.len() <= PAYLOAD_MAX, "{} held", a.rx.len());
            if a.rx.is_empty() && a.end.is_none() {
                assert_eq!(a.bits(false, false) & POLLIN, 0, "readable on an empty open stream");
            }
        }
    }
}

#[test]
fn a_fill_longer_than_a_reply_is_a_broken_answer() {
    let mut a = open();
    a.fill(Got::Bytes(vec![0; PAYLOAD_MAX + 1]));
    assert!(a.rx.is_empty());
    assert_eq!(a.end, Some(End::Error(EIO)));
    assert_eq!(a.take(1, false, false), Err(EIO));
    assert_eq!(a.take(1, false, false), Ok(Vec::new()), "reported once, then end of file");
    let mut a = open();
    a.fill(Got::Bytes(vec![0; PAYLOAD_MAX]));
    assert_eq!(a.rx.len(), PAYLOAD_MAX, "a whole reply is taken");
}

#[test]
fn a_clean_end_is_end_of_file_readable_and_not_an_error() {
    let mut a = open();
    a.fill(Got::Bytes(vec![9]));
    let _ = a.take(1, false, false);
    a.fill(Got::End(REASON_DONE));
    let bits = a.bits(false, false);
    assert_eq!(bits & (POLLIN | POLLRDHUP), POLLIN | POLLRDHUP);
    assert_eq!(bits & (POLLERR | POLLHUP), 0);
    assert_eq!(a.take(8, false, false), Ok(Vec::new()));
    assert_eq!(a.take(8, false, false), Ok(Vec::new()));
}

#[test]
fn a_stream_ended_before_any_byte_was_never_connected() {
    for (reason, want) in [
        (REASON_CONNECTREFUSED, ECONNREFUSED),
        (REASON_EXITPOLICY, ECONNREFUSED),
        (REASON_DESTROY, ENETUNREACH),
        (REASON_TIMEOUT, ETIMEDOUT),
        (0, ECONNREFUSED),
        (u8::MAX, ECONNREFUSED),
    ] {
        let mut a = open();
        a.fill(Got::End(reason));
        assert_eq!(a.bits(false, false) & (POLLIN | POLLERR | POLLHUP), POLLIN | POLLERR | POLLHUP);
        assert_eq!(a.take(8, false, false), Err(want), "reason {reason}");
        assert_eq!(a.take(8, false, false), Ok(Vec::new()), "reported once");
    }
}

#[test]
fn a_stream_that_ended_after_bytes_came_was_reset() {
    for reason in [REASON_CONNECTREFUSED, REASON_DESTROY, REASON_TIMEOUT, 1] {
        let mut a = open();
        a.fill(Got::Bytes(vec![1, 2]));
        assert_eq!(a.take(2, false, false), Ok(vec![1, 2]));
        a.fill(Got::End(reason));
        assert_eq!(a.take(8, false, false), Err(ECONNRESET), "reason {reason}");
    }
}

#[test]
fn a_stream_net_anon_does_not_hold_or_that_does_not_answer_ends() {
    for (got, want) in [(Got::Gone, ECONNRESET), (Got::Garbled, EIO), (Got::Silent, EIO)] {
        let mut a = open();
        a.fill(got);
        assert_eq!(a.take(8, false, false), Err(want));
        assert!(!a.wants_fill(), "nothing more is asked of net.anon");
    }
}

#[test]
fn shutting_the_reading_side_reads_end_of_file() {
    let mut a = open();
    a.fill(Got::Bytes(vec![5; 3]));
    assert_eq!(a.take(3, false, true), Ok(Vec::new()));
    let bits = a.bits(true, false);
    assert_eq!(bits & (POLLIN | POLLRDHUP | POLLHUP), POLLIN | POLLRDHUP);
    assert_eq!(a.bits(true, true) & POLLHUP, POLLHUP, "both sides shut");
}

#[test]
fn a_read_of_nothing_is_nothing() {
    let mut a = open();
    assert_eq!(a.take(0, false, false), Ok(Vec::new()));
}
