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

//! mremap's and munmap's arguments, and a span's end.

use super::mmap::{DONTUNMAP, MAYMOVE, MFIXED};
use crate::linux::abi::errno::EINVAL;
use crate::linux::call::mem::map_args::flags::MIN_ADDR;
use crate::linux::call::mem::remap_args::{end_of, remap_args, unmap_span};
use crate::linux::guest::{PAGE, USER_MAX};

#[test]
fn mremap_refuses_each_bad_argument_with_einval() {
    let at = 0x10_0000;
    // (old, old_len, new_len, flags)
    let refused: [(u64, u64, u64, u64); 11] = [
        (at, PAGE, 2 * PAGE, 8),
        (at, PAGE, 2 * PAGE, MFIXED),
        (at, PAGE, 2 * PAGE, MFIXED | MAYMOVE),
        (at, PAGE, PAGE, DONTUNMAP),
        (at, PAGE, 2 * PAGE, DONTUNMAP | MAYMOVE),
        (at, PAGE, PAGE, DONTUNMAP | MAYMOVE),
        (at + 8, PAGE, 2 * PAGE, MAYMOVE),
        (at, PAGE, 0, MAYMOVE),
        (at, 0, PAGE, MAYMOVE),
        // Lengths that round past the top: PAGE_ALIGN wraps them to zero.
        (at, PAGE, u64::MAX, MAYMOVE),
        (at, u64::MAX - 1, PAGE, MAYMOVE),
    ];
    for (old, old_len, new_len, flags) in refused {
        assert_eq!(remap_args(old, old_len, new_len, flags), Err(EINVAL), "{old_len:#x} {flags}");
    }
    assert_eq!(remap_args(at, 1, PAGE + 1, MAYMOVE), Ok((PAGE, 2 * PAGE)));
    assert_eq!(remap_args(at, 3 * PAGE, PAGE, 0), Ok((3 * PAGE, PAGE)));
}

#[test]
fn a_span_end_never_wraps_or_passes_the_guest_area() {
    assert_eq!(end_of(USER_MAX - PAGE, PAGE), Some(USER_MAX));
    assert_eq!(end_of(USER_MAX, 1), None);
    assert_eq!(end_of(1 << 63, 1 << 63), None);
    assert_eq!(end_of(MIN_ADDR, u64::MAX - MIN_ADDR + PAGE), None, "wraps to a low address");
}

#[test]
fn munmap_refuses_each_bad_argument_with_einval() {
    let at = 0x10_0000;
    for (addr, len) in
        [(at + 1, PAGE), (at, 0), (USER_MAX, 1), (at, u64::MAX), (u64::MAX - PAGE + 1, PAGE)]
    {
        assert_eq!(unmap_span(addr, len), Err(EINVAL), "{addr:#x} {len:#x}");
    }
    assert_eq!(unmap_span(at, 1), Ok((at, PAGE)));
    assert_eq!(unmap_span(USER_MAX - PAGE, PAGE), Ok((USER_MAX - PAGE, PAGE)));
}
