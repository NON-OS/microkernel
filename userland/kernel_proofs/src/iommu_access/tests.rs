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

use super::access::{RemapUnit, UNIT_WINDOW};
use std::panic::{catch_unwind, AssertUnwindSafe};

fn unit(page: &[u64]) -> RemapUnit {
    // SAFETY: page is a live buffer of UNIT_WINDOW bytes for this test.
    unsafe { RemapUnit::from_mapped(page.as_ptr() as u64, 0xFED9_0000) }
}

#[test]
fn the_last_register_in_the_window_is_read() {
    let mut page = vec![0u64; UNIT_WINDOW / 8];
    *page.last_mut().unwrap() = 0x1122_3344_5566_7788;
    let u = unit(&page);
    assert_eq!(u.read64(UNIT_WINDOW - 8), 0x1122_3344_5566_7788);
    assert_eq!(u.read32(UNIT_WINDOW - 4), 0x1122_3344);
}

#[test]
fn offsets_past_the_window_are_refused_even_when_they_would_wrap() {
    let page = vec![0u64; UNIT_WINDOW / 8];
    let u = unit(&page);
    for offset in [UNIT_WINDOW - 3, UNIT_WINDOW, usize::MAX - 3, usize::MAX] {
        assert!(catch_unwind(AssertUnwindSafe(|| u.read32(offset))).is_err(), "{offset:#x}");
    }
    for offset in [UNIT_WINDOW - 7, usize::MAX - 7, usize::MAX] {
        assert!(catch_unwind(AssertUnwindSafe(|| u.read64(offset))).is_err(), "{offset:#x}");
    }
}
