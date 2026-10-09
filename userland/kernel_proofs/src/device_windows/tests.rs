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

use super::windows::{
    touches_device_window, USER_DMA_BASE, USER_DMA_END, USER_MMIO_BASE, USER_MMIO_END,
};

const PAGE: u64 = 4096;
/// The kernel's user ceiling (syscall/microkernel/memory/consts.rs).
const USER_SPACE_MAX: u64 = 0x0000_7FFF_FFFF_FFFF;

#[test]
fn every_page_of_either_window_is_refused() {
    for (lo, hi) in [(USER_MMIO_BASE, USER_MMIO_END), (USER_DMA_BASE, USER_DMA_END)] {
        assert!(touches_device_window(lo, PAGE));
        assert!(touches_device_window(hi - PAGE, PAGE));
        assert!(touches_device_window(lo + (hi - lo) / 2, PAGE));
        // A range that starts below and reaches in, or covers it whole.
        assert!(touches_device_window(lo - PAGE, 2 * PAGE));
        assert!(touches_device_window(lo - PAGE, hi - lo + 2 * PAGE));
    }
}

#[test]
fn the_pages_beside_the_windows_are_untouched() {
    for (lo, hi) in [(USER_MMIO_BASE, USER_MMIO_END), (USER_DMA_BASE, USER_DMA_END)] {
        assert!(!touches_device_window(lo - PAGE, PAGE));
        assert!(!touches_device_window(hi, PAGE));
    }
    // Ordinary heap, image and stack ranges.
    assert!(!touches_device_window(0x40_0000, 16 * PAGE));
    assert!(!touches_device_window(0x8000_0000, 1 << 30));
    assert!(!touches_device_window(0x0000_7FFF_FFFF_0000, PAGE));
}

#[test]
fn a_wrapping_range_is_refused() {
    assert!(touches_device_window(u64::MAX - PAGE, 2 * PAGE));
}

#[test]
fn the_windows_are_inside_user_space_and_disjoint() {
    const { assert!(USER_MMIO_BASE < USER_MMIO_END && USER_MMIO_END <= USER_DMA_BASE) };
    const { assert!(USER_DMA_BASE < USER_DMA_END && USER_DMA_END <= USER_SPACE_MAX) };
}

#[test]
fn a_random_range_is_refused_exactly_when_it_overlaps() {
    let mut s = 0x0123_4567_89AB_CDEFu64;
    for _ in 0..200_000 {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        let addr = (s % 0x0000_00C0_0000_0000) & !(PAGE - 1);
        let len = ((s >> 40) % 4096 + 1) * PAGE;
        let end = addr + len;
        let want = (addr < USER_MMIO_END && end > USER_MMIO_BASE)
            || (addr < USER_DMA_END && end > USER_DMA_BASE);
        assert_eq!(touches_device_window(addr, len), want, "{addr:#x}+{len:#x}");
    }
}
