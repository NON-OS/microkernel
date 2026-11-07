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

//! MkPeerReadRows (src/process/foreign/peer_rows.rs): a layout is taken
//! only when its rows are within their stride, there are at most MAX_ROWS of
//! them holding at most the call's bytes, and its span fits; every row it
//! names lies inside that span, so the guest range checked is all a call
//! reads. The source checks pin the call to the supervisor's gate, to that
//! layout, and to reading the guest only.

use super::peer_rows_shape::{Rows, MAX_ROWS};

const MAX: u64 = 1 << 20;
const SOURCE: &str = include_str!("../../../../src/process/foreign/peer_rows.rs");
const GUARD: &str = include_str!("../../../../src/process/foreign/peer_guard.rs");

#[test]
fn a_layout_is_taken_only_within_its_limits() {
    assert!(Rows::new(480, 40, 7680, MAX).is_some(), "a button of a 1080p window");
    assert_eq!(Rows::new(0, 1, 4, MAX), None, "an empty row");
    assert_eq!(Rows::new(8, 1, 4, MAX), None, "a row wider than its stride");
    assert_eq!(Rows::new(4, 0, 4, MAX), None, "no rows");
    assert_eq!(Rows::new(4, MAX_ROWS + 1, 4, MAX), None, "more rows than a call walks");
    assert!(Rows::new(4, MAX_ROWS, 4, MAX).is_some());
    assert_eq!(Rows::new(MAX / 2 + 1, 2, MAX, MAX), None, "more bytes than a call moves");
    assert!(Rows::new(MAX / 2, 2, MAX, MAX).is_some());
    assert_eq!(Rows::new(1, 2, u64::MAX, MAX), None, "a span past u64");
    assert_eq!(Rows::new(u64::MAX, 2, u64::MAX, MAX), None, "row bytes past u64");
}

#[test]
fn every_row_lies_inside_the_span_and_no_two_rows_meet() {
    for (row, count, stride) in [(1, 1, 1), (4, 3, 4), (480, 40, 7680), (5, 4096, 9), (7, 9, 7)] {
        let r = Rows::new(row, count, stride, MAX).expect("taken");
        let span = r.span().unwrap();
        assert_eq!(r.bytes(), row * count);
        for i in 0..count {
            let at = r.at(i).unwrap();
            assert!(at + row <= span, "{row}x{count}@{stride}: row {i} inside the span");
            if i > 0 {
                assert!(r.at(i - 1).unwrap() + row <= at, "row {i} after the one before");
            }
        }
        assert_eq!(r.at(count), None, "past the last row");
    }
}

#[test]
fn the_call_is_held_to_its_supervisor_its_layout_and_reading_the_guest() {
    let body = SOURCE.split("pub fn sys_peer_read_rows").nth(1).expect("the call");
    let gate = body.find("supervised_asid(caller, pid)").expect("the supervisor's gate");
    let shape = body.find("Rows::new(row, count, stride, MAX_SPAN)").expect("the layout");
    let half = body.find("in_user_half(guest_addr, span)").expect("the guest's half");
    let caller = body.find("rows_writable(&rows, buf)").expect("the caller's rows");
    let first_read = body.find("read_row(").expect("the rows read");
    assert!(
        gate < shape && shape < half && half < caller && caller < first_read,
        "checked before any page"
    );
    let writable = SOURCE.split("fn rows_writable").nth(1).expect("the caller's check");
    assert!(writable.contains("validate(buf + at, rows.row, false)"), "each row, for writing");
    assert!(body.contains("buf.checked_add(span).is_none()"), "the caller's span cannot wrap");
    assert!(SOURCE.contains("chunk_copy(phys, offset, buf + done, chunk, false)"));
    assert!(!SOURCE.contains(", true)"), "never writes the guest");
    assert!(!SOURCE.contains("unsafe"), "no unsafe of its own");
}

#[test]
fn the_bridge_cuts_its_reads_at_the_kernels_limits() {
    use super::bridge_rows::{calls, ROWS_MAX, ROWS_MAX_BYTES};
    assert_eq!(ROWS_MAX as u64, MAX_ROWS, "rows a call");
    assert_eq!(ROWS_MAX_BYTES as u64, MAX, "bytes a call");
    assert!(GUARD.contains("const MAX_SPAN: u64 = 1 << 20;"), "MAX is the kernel's MAX_SPAN");
    for (row, count) in [(1, 9000), (4, 4097), (480, 1080), (7680, 1080), (1 << 20, 3)] {
        let cut = calls(row, count).expect("a row within one call");
        let mut next = 0;
        for (first, n) in cut {
            assert_eq!(first, next, "{row} x {count}: rows in order, none skipped");
            assert!(Rows::new(row as u64, n as u64, row as u64, MAX).is_some(), "{row} x {n}");
            next = first + n;
        }
        assert_eq!(next, count, "{row} x {count}: every row read");
    }
    assert!(calls((1 << 20) + 1, 1).is_none(), "a row past one call is read as its own span");
}
