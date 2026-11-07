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

//! Any row, count, stride and byte bound as MkPeerReadRows's layout, the
//! kernel's own src/process/foreign/peer_rows_shape.rs mounted unchanged. A
//! layout is taken exactly when a reference in u128 says it may be: rows not
//! empty and no wider than their stride, 1 to MAX_ROWS of them, their bytes
//! within the bound and the span from the first byte to the last within a
//! u64. A taken layout puts every row inside its span, apart from the next,
//! at the same offset the syscall copies it to, and names no row past the last.

#![no_main]

use libfuzzer_sys::fuzz_target;

#[path = "../../../../src/process/foreign/peer_rows_shape.rs"]
mod peer_rows_shape;
use peer_rows_shape::{Rows, MAX_ROWS};

fn word(b: &[u8], i: usize) -> u64 {
    let mut w = [0u8; 8];
    for (k, x) in w.iter_mut().enumerate() {
        *x = b.get(i * 8 + k).copied().unwrap_or(0);
    }
    u64::from_le_bytes(w)
}

/// Whether the layout may be taken, worked in u128 where nothing wraps.
fn allowed(row: u64, count: u64, stride: u64, max: u64) -> bool {
    let (r, c, s) = (u128::from(row), u128::from(count), u128::from(stride));
    row != 0
        && row <= stride
        && (1..=MAX_ROWS).contains(&count)
        && r * c <= u128::from(max)
        && s * (c - 1) + r <= u128::from(u64::MAX)
}

fuzz_target!(|data: &[u8]| {
    // Small values reach the edges more often than any u64 does.
    let shape = |x: u64, sel: u8| match sel % 4 {
        0 => x,
        1 => x % 64,
        2 => x % (1 << 21),
        _ => u64::MAX - x % 64,
    };
    let sel = data.get(32).copied().unwrap_or(0);
    let row = shape(word(data, 0), sel);
    let count = shape(word(data, 1), sel >> 2);
    let stride = shape(word(data, 2), sel >> 4);
    let max = shape(word(data, 3), sel >> 6);
    let got = Rows::new(row, count, stride, max);
    assert_eq!(got.is_some(), allowed(row, count, stride, max), "{row} {count} {stride} {max}");
    let Some(r) = got else {
        return;
    };
    assert_eq!((r.row, r.count, r.stride), (row, count, stride));
    assert_eq!(u128::from(r.bytes()), u128::from(row) * u128::from(count));
    let span = r.span().expect("a taken layout has a span");
    // Every row is checked when there are few; the first and last few else.
    let rows: Vec<u64> = if count <= 64 {
        (0..count).collect()
    } else {
        (0..8).chain(count - 8..count).collect()
    };
    for i in rows {
        let at = r.at(i).expect("a row before the last has a place");
        assert!(at.checked_add(row).is_some_and(|end| end <= span), "row {i} past the span");
        if let Some(next) = r.at(i + 1) {
            assert!(next >= at + row, "row {i} runs into the next");
        }
    }
    assert_eq!(r.at(count), None, "a row past the last");
    assert_eq!(r.at(count - 1).map(|at| at + row), Some(span), "the span ends with the last row");
});
