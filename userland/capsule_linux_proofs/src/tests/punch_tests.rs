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

//! fallocate's PUNCH_HOLE zeroes only the bytes the held copy has. The
//! zeroes are one allocation, so a span cut to anything larger, such as a
//! length a refused ftruncate asked for, is an allocation the guest sized.

use super::random::Regs;
use crate::linux::file::punch::punched;

/// What the family may hold of one file (held/cache/table.rs MAX_FILE).
const MAX_FILE: u64 = 8 << 20;

#[test]
fn a_hole_inside_the_file_is_zeroed_as_asked() {
    assert_eq!(punched(0, 4096, 8192), Some((0, 4096)));
    assert_eq!(punched(100, 50, 8192), Some((100, 150)));
}

#[test]
fn a_hole_past_the_end_is_cut_to_the_end() {
    assert_eq!(punched(4096, 1 << 40, 8192), Some((4096, 8192)));
    assert_eq!(punched(0, u64::MAX, 10), Some((0, 10)), "a length that wraps is cut, not wrapped");
}

/// The case the stale length made: a guest's ftruncate to a terabyte is
/// refused, the name is unlinked, and the hole it then punches is sized
/// against the copy held at that moment, which is empty.
#[test]
fn nothing_outside_the_held_copy_is_zeroed() {
    assert_eq!(punched(0, 1 << 40, 0), None, "an empty copy has nothing to zero");
    assert_eq!(punched(8192, 1, 8192), None, "a hole starting at the end");
    assert_eq!(punched(u64::MAX, u64::MAX, 4096), None);
}

/// Over any arguments, the span is inside the copy, never empty, and never
/// larger than the most a family holds of one file.
#[test]
fn no_arguments_make_the_zeroes_larger_than_the_copy() {
    let mut r = Regs::new(0xFA11_0CA7);
    for _ in 0..200_000 {
        let (at, len) = (r.arg(), r.arg());
        let size = r.small(MAX_FILE + 1);
        if let Some((from, to)) = punched(at, len, size) {
            assert!(from < to && to <= size, "{at:#x}+{len:#x} in {size}: {from}..{to}");
            assert!(to - from <= MAX_FILE);
            assert_eq!(from, at, "a hole starts where it was asked to");
        }
    }
}
