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

//! the claim-time read of bit 33 over a device's registers. The select
//! is restored on every path that moved it, so the driver inherits nothing
//! it did not write; a device that does not answer, or whose select does
//! not take, is unread, and so refused.

use std::cell::{Cell, RefCell};
use std::vec::Vec;

use super::virtio_read::{access_platform, FEATURE, FEATURE_SELECT};

/// A common window: feature words by select, a select that may not stick,
/// and every write it saw.
fn run(words: [u32; 2], start: u32, sticks: bool) -> (Option<bool>, u32, Vec<(u64, u32)>) {
    let select = Cell::new(start);
    let writes = RefCell::new(Vec::new());
    let read = |r: u64| match r {
        FEATURE_SELECT => select.get(),
        FEATURE => words.get(select.get() as usize).copied().unwrap_or(0),
        _ => 0,
    };
    let write = |r: u64, v: u32| {
        writes.borrow_mut().push((r, v));
        if r == FEATURE_SELECT && (sticks || v == start) {
            select.set(v);
        }
    };
    let got = access_platform(read, write);
    (got, select.get(), writes.into_inner())
}

#[test]
fn the_bit_is_read_and_the_select_put_back() {
    assert_eq!(run([0, 1 << 1], 0, true).0, Some(true));
    assert_eq!(run([0, 0], 0, true).0, Some(false));
    let (got, left, writes) = run([0, 1 << 1], 0, true);
    assert_eq!((got, left), (Some(true), 0), "select back at 0");
    assert_eq!(writes, [(FEATURE_SELECT, 1), (FEATURE_SELECT, 0)]);
}

#[test]
fn a_device_that_does_not_answer_or_keep_its_select_is_unread() {
    let (got, left, writes) = run([0, 0], u32::MAX, true);
    assert_eq!((got, left), (None, u32::MAX), "all ones: nothing written");
    assert!(writes.is_empty());
    let (got, left, _) = run([0, 1 << 1], 0, false);
    assert_eq!((got, left), (None, 0), "a select that does not take, then restored");
    let (got, left, _) = run([0, u32::MAX], 0, true);
    assert_eq!((got, left), (None, 0), "an all-ones feature word, select restored");
}
