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

//! A stream net.anon lets go, a reading side shut, and a read of nothing.

use super::held::{open, POLLHUP, POLLIN, POLLRDHUP};
use crate::linux::abi::errno::{ECONNRESET, EIO};
use crate::linux::net::sock::Got;

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
