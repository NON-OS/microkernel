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

//! How a damaged box is read out of the guest (wayland/damage_box.rs): the
//! reads planned are exactly the box's bytes, no byte between its rows and
//! no byte twice, in one read when its rows meet; no read passes the bytes
//! the client's buffer has.

use std::collections::BTreeSet;

use crate::damage_box::{plan, Corners, Reads};

/// The bytes of box `b` in a buffer at `stride` bytes a row.
fn box_bytes((x0, y0, x1, y1): Corners, stride: usize) -> BTreeSet<usize> {
    let row = |y: u32| (x0 as usize * 4..x1 as usize * 4).map(move |x| y as usize * stride + x);
    (y0..y1).flat_map(row).collect()
}

fn read_bytes(r: &Reads) -> BTreeSet<usize> {
    (0..r.count).flat_map(|i| r.at(i).map(|at| at..at + r.len)).flatten().collect()
}

/// Every box of a `w` by `h` buffer, empty ones and ones past its edge too.
fn boxes(w: u32, h: u32) -> impl Iterator<Item = Corners> {
    (0..=w + 1).flat_map(move |x0| {
        (0..=h + 1).flat_map(move |y0| {
            (0..=w + 1).flat_map(move |x1| (0..=h + 1).map(move |y1| (x0, y0, x1, y1)))
        })
    })
}

#[test]
fn a_box_is_read_as_its_own_bytes_and_no_other() {
    let mut seen = [0usize; 2];
    for (w, h) in [(1, 1), (3, 2), (5, 4), (6, 3)] {
        for pad in [0, 4, 12] {
            let stride = w * 4 + pad;
            let read = (h as usize - 1) * stride as usize + w as usize * 4;
            for b in boxes(w, h) {
                let on = b.0 < b.2 && b.1 < b.3 && b.2 <= w && b.3 <= h;
                let Some(r) = plan((w, h, stride), read, b) else {
                    assert!(!on, "{b:?} of {w}x{h} at {stride} refused");
                    continue;
                };
                assert!(on, "{b:?} off {w}x{h} planned");
                let want = box_bytes(b, stride as usize);
                assert_eq!(read_bytes(&r), want, "{b:?} of {w}x{h} at {stride}");
                assert_eq!(r.bytes(), want.len(), "{b:?}: no byte read twice");
                let span = r.span().unwrap();
                assert_eq!(r.first + span - 1, *want.last().unwrap(), "{b:?}: to its last");
                assert!(r.first + span <= read, "{b:?} inside the buffer");
                match r.count {
                    1 => seen[0] += 1,
                    _ => {
                        seen[1] += 1;
                        assert_eq!(r.step, stride as usize, "{b:?}: rows at the stride");
                        assert!(r.len < r.step, "{b:?}: rows that meet are one read");
                    }
                }
            }
        }
    }
    assert!(seen[0] > 0 && seen[1] > 0, "both shapes made: {seen:?}");
}

#[test]
fn a_box_narrower_than_its_stride_is_its_rows_and_a_full_width_one_is_one_read() {
    let (w, h, stride) = (1920, 1080, 7680);
    let read = 1080 * 7680;
    let button = plan((w, h, stride), read, (960, 540, 1080, 580)).unwrap();
    assert_eq!(button, Reads { first: 540 * 7680 + 3840, step: 7680, len: 480, count: 40 });
    let pane = plan((w, h, stride), read, (480, 48, 1920, 1080)).unwrap();
    assert_eq!(pane, Reads { first: 48 * 7680 + 1920, step: 7680, len: 5760, count: 1032 });
    assert_eq!(pane.bytes(), 1032 * 5760, "none of the 1920 bytes beside each row");
    let scroll = plan((w, h, stride), read, (0, 48, 1920, 1080)).unwrap();
    assert_eq!(scroll, Reads { first: 48 * 7680, step: 0, len: 1032 * 7680, count: 1 });
}

#[test]
fn full_rows_with_bytes_between_them_are_read_without_those_bytes() {
    // A window geometry's box: the stride is the buffer's, wider than the box.
    let (w, h, stride) = (800, 600, 852 * 4);
    let read = 599 * stride as usize + 800 * 4;
    let r = plan((w, h, stride), read, (0, 0, 800, 600)).unwrap();
    assert_eq!(r, Reads { first: 0, step: 852 * 4, len: 3200, count: 600 });
    let r = plan((w, h, stride), read, (0, 10, 300, 20)).unwrap();
    assert!(r.count == 10 && r.len == 1200, "{r:?}");
}

#[test]
fn a_read_past_what_the_client_has_or_a_stride_under_a_row_is_refused() {
    let (w, h, stride) = (64, 32, 256);
    let whole = 32 * 256;
    assert!(plan((w, h, stride), whole, (0, 31, 64, 32)).is_some());
    assert_eq!(plan((w, h, stride), whole - 1, (0, 31, 64, 32)), None, "one byte short");
    assert_eq!(plan((w, h, stride), whole - 1, (60, 31, 64, 32)), None, "its last pixel short");
    assert_eq!(plan((w, h, 252), whole, (0, 0, 1, 1)), None, "a stride under its row");
    assert_eq!(plan((w, h, 0), whole, (0, 0, 1, 1)), None, "no stride");
}

#[test]
fn shapes_at_the_ends_of_their_ranges_are_answered_without_wrapping() {
    let max = u32::MAX;
    for shape in [(max, max, max), (max / 4, max, max), (1, max, max), (max, 1, max / 2)] {
        for b in [(0, 0, max, max), (max - 1, max - 1, max, max), (0, max - 2, 1, max)] {
            if let Some(r) = plan(shape, usize::MAX, b) {
                assert!(r.first.checked_add(r.span().unwrap()).is_some(), "{shape:?} {b:?}");
            }
            assert_eq!(plan(shape, 0, b), None, "{shape:?} {b:?}: nothing the client's");
        }
    }
    let r = Reads { first: usize::MAX, step: 1, len: 1, count: 3 };
    assert_eq!(r.at(1), None, "a step past the end is no offset");
    assert_eq!(r.at(3), None, "past the last read");
    assert_eq!(Reads { first: 0, step: usize::MAX, len: 2, count: 2 }.span(), None);
    assert_eq!(Reads { first: 0, step: 4, len: 4, count: 0 }.span(), None, "no reads");
}
