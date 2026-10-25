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

//! How a stream ends: cleanly, never connected, or reset.

use super::held::{
    open, POLLERR, POLLHUP, POLLIN, POLLRDHUP, REASON_CONNECTREFUSED, REASON_EXITPOLICY,
};
use crate::linux::abi::errno::{ECONNREFUSED, ECONNRESET, ENETUNREACH, ETIMEDOUT};
use crate::linux::net::anon_ops::{REASON_DESTROY, REASON_DONE, REASON_TIMEOUT};
use crate::linux::net::sock::Got;

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
