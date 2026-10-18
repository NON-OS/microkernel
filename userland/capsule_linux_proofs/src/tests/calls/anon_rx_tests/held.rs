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

//! What an open stream holds, and what a read takes.

use crate::linux::abi::errno::EAGAIN;
use crate::linux::net::anon_ops::REASON_DONE;
use crate::linux::net::sock::{Anon, Got};

pub(super) const POLLIN: u16 = 0x001;
const POLLOUT: u16 = 0x004;
pub(super) const POLLERR: u16 = 0x008;
pub(super) const POLLHUP: u16 = 0x010;
pub(super) const POLLRDHUP: u16 = 0x2000;

/// tor-spec 6.3: the exit or the host refused, the exit's policy refused.
pub(super) const REASON_CONNECTREFUSED: u8 = 3;
pub(super) const REASON_EXITPOLICY: u8 = 4;

pub(super) fn open() -> Anon {
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
