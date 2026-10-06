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

//! The doorbell offset: base + queue_notify_off * multiplier, bounded.

use crate::mmio::Mmio;
use crate::notify::notify_offset;

#[test]
fn multiplier_four_spaces_doorbells_four_bytes_apart() {
    assert_eq!(notify_offset(0, 4, 0x1000), Some(0));
    assert_eq!(notify_offset(1, 4, 0x1000), Some(4));
    assert_eq!(notify_offset(1023, 4, 0x1000), Some(4092));
    assert_eq!(notify_offset(1024, 4, 0x1000), None, "the write would end past the region");
}

#[test]
fn multiplier_zero_puts_every_queue_on_one_doorbell() {
    for q in [0u16, 1, 7, u16::MAX] {
        assert_eq!(notify_offset(q, 0, 2), Some(0));
    }
    assert_eq!(notify_offset(0, 0, 1), None, "a one-byte region holds no 16-bit doorbell");
    assert_eq!(notify_offset(0, 0, 0), None);
}

#[test]
fn the_last_two_bytes_are_the_last_doorbell() {
    assert_eq!(notify_offset(1, 0x0FFE, 0x1000), Some(0x0FFE));
    assert_eq!(notify_offset(1, 0x0FFF, 0x1001), None, "odd: off a 16-bit boundary");
    assert_eq!(notify_offset(1, 0x1000, 0x1000), None);
}

#[test]
fn the_largest_device_values_do_not_wrap() {
    // u16::MAX * u32::MAX is about 2^48: exact in 64 bits, far past any
    // mapped region, refused rather than wrapped back into it.
    assert_eq!(notify_offset(u16::MAX, u32::MAX, 0x4000), None);
    assert_eq!(notify_offset(u16::MAX, 0x1000, 0x4000), None);
    for len in [0usize, 2, 0x1000, 0x4000, usize::MAX / 2] {
        if let Some(off) = notify_offset(u16::MAX, u32::MAX - 1, len) {
            assert!(off as u64 + 2 <= len as u64);
        }
    }
}

#[test]
fn every_accepted_offset_is_a_whole_aligned_doorbell_inside_the_region() {
    let mut s = 0x0123_4567_89AB_CDEFu64;
    for _ in 0..200_000 {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        let q = s as u16;
        let mult = ((s >> 16) as u32) >> ((s >> 48) & 31);
        let len = (s >> 40) as usize & 0x7FFF;
        if let Some(off) = notify_offset(q, mult, len) {
            assert_eq!(off as u64, q as u64 * mult as u64);
            assert!(off + 2 <= len);
            assert_eq!(off % 2, 0);
        }
    }
}

#[repr(align(4096))]
struct Page([u8; 4096]);

#[test]
fn a_doorbell_write_through_the_region_cannot_leave_it() {
    // The full path the drivers take: the offset, then the 16-bit write
    // through the mapped notify region. A refused offset never reaches the
    // region, and the region refuses an offset that would wrap the address.
    let mut page = Box::new(Page([0; 4096]));
    let area = unsafe { Mmio::new(page.0.as_mut_ptr().add(0x100), 0x10) };
    let off = notify_offset(3, 4, area.len()).expect("inside");
    area.w16(off, 0xBEEF);
    assert_eq!(page.0[0x10C..0x10E], [0xEF, 0xBE]);
    assert_eq!(notify_offset(4, 4, area.len()), None);
    area.w16(usize::MAX - 1, 0x5555);
    area.w16(usize::MAX, 0x5555);
    area.w16(0x10, 0x5555);
    assert!(page.0[..0x10C].iter().all(|&b| b == 0));
    assert!(page.0[0x10E..].iter().all(|&b| b == 0));
}
