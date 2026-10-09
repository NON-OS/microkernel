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

//! The region accessors: inside, little-endian and aligned, or nothing.

use crate::common::{CommonCfg, DEVICE_STATUS, QUEUE_DESC};
use crate::mmio::Mmio;

#[repr(align(4096))]
struct Page([u8; 4096]);

fn page() -> Box<Page> {
    Box::new(Page([0; 4096]))
}

#[test]
fn reads_and_writes_land_at_their_offsets() {
    let mut p = page();
    let r = unsafe { Mmio::new(p.0.as_mut_ptr().add(0x40), 0x40) };
    r.w32(0, 0x1122_3344);
    r.w16(4, 0x5566);
    r.w8(6, 0x77);
    assert_eq!(p.0[0x40..0x47], [0x44, 0x33, 0x22, 0x11, 0x66, 0x55, 0x77]);
    assert_eq!(r.r32(0), 0x1122_3344);
    assert_eq!(r.r16(4), 0x5566);
    assert_eq!(r.r8(6), 0x77);
}

#[test]
fn outside_the_region_reads_all_ones_and_writes_nothing() {
    let mut p = page();
    let r = unsafe { Mmio::new(p.0.as_mut_ptr().add(0x40), 0x10) };
    p.0[0x50] = 0xAA;
    assert_eq!(r.r8(0x10), u8::MAX);
    assert_eq!(r.r16(0x0F), u16::MAX, "straddles the end");
    assert_eq!(r.r32(0x0D), u32::MAX);
    assert_eq!(r.r32(usize::MAX - 2), u32::MAX, "the offset sum would wrap");
    r.w8(0x10, 1);
    r.w16(0x0F, 1);
    r.w32(0x0E, 1);
    r.w32(usize::MAX, 1);
    assert_eq!(p.0[0x50], 0xAA);
    assert!(p.0[..0x40].iter().all(|&b| b == 0));
}

#[test]
fn a_misaligned_access_is_refused() {
    let mut p = page();
    let r = unsafe { Mmio::new(p.0.as_mut_ptr(), 0x40) };
    r.w32(2, 0xFFFF_FFFF);
    r.w16(1, 0xFFFF);
    assert!(p.0.iter().all(|&b| b == 0));
    assert_eq!(r.r32(1), u32::MAX);
    assert_eq!(r.r16(3), u16::MAX);
    // The base's own alignment counts: a region starting two bytes into a
    // page has no aligned 32-bit register at offset 0.
    let odd = unsafe { Mmio::new(p.0.as_mut_ptr().add(2), 0x10) };
    odd.w32(0, 0xFFFF_FFFF);
    assert!(p.0.iter().all(|&b| b == 0));
    odd.w16(0, 0x1234);
    assert_eq!(p.0[2..4], [0x34, 0x12]);
}

#[test]
fn a_64_bit_register_is_written_low_half_first() {
    let mut p = page();
    let r = unsafe { Mmio::new(p.0.as_mut_ptr(), 0x38) };
    r.w64(QUEUE_DESC, 0x0000_0001_2345_6000);
    assert_eq!(r.r32(QUEUE_DESC), 0x2345_6000);
    assert_eq!(r.r32(QUEUE_DESC + 4), 0x1);
    r.w8(DEVICE_STATUS, 0x0F);
    assert_eq!(CommonCfg::r8(&r, DEVICE_STATUS), 0x0F);
}
