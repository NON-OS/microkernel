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

//! How a strided read of a guest's rows is cut into kernel calls
//! (guest/mem_rows.rs): every row in exactly one call, in order, and no
//! call past the kernel's limits (src/process/foreign/peer_rows_shape.rs).

use crate::mem_rows::{calls, rows_a_call, ROWS_MAX, ROWS_MAX_BYTES};

#[test]
fn every_row_is_in_one_call_and_no_call_passes_the_kernels_limits() {
    for row in [1, 4, 480, 3840, 5760, 7680, 65536, ROWS_MAX_BYTES] {
        for count in [0, 1, 2, 40, 136, 137, 1032, 4096, 4097, 9000] {
            let got: Vec<_> = calls(row, count).expect("a row a call fits").collect();
            let mut next = 0;
            for &(first, n) in &got {
                assert_eq!(first, next, "row {row} x {count}: in order, none skipped");
                assert!((1..=ROWS_MAX).contains(&n), "row {row} x {count}: {n} rows a call");
                assert!(n * row <= ROWS_MAX_BYTES, "row {row} x {count}: {} bytes", n * row);
                next += n;
            }
            assert_eq!(next, count, "row {row} x {count}: every row");
            let fewest = count.div_ceil(rows_a_call(row));
            assert_eq!(got.len(), fewest, "row {row} x {count}: as few calls as fit");
        }
    }
}

#[test]
fn a_box_of_a_1080p_window_is_one_call_or_a_handful() {
    assert_eq!(calls(480, 40).unwrap().count(), 1, "a button");
    assert_eq!(calls(5760, 1032).unwrap().count(), 6, "a chat pane, 1032 rows of 5760 bytes");
    assert_eq!(calls(1056, 256).unwrap().count(), 1, "a 264 pixel wide drag");
}

#[test]
fn an_empty_row_or_one_wider_than_a_call_is_not_cut() {
    assert_eq!(rows_a_call(0), 0);
    assert!(calls(0, 4).is_none());
    assert_eq!(rows_a_call(ROWS_MAX_BYTES + 1), 0);
    assert!(calls(ROWS_MAX_BYTES + 1, 4).is_none());
}
