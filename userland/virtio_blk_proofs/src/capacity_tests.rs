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

// The capacity a modern virtio-blk reports comes out of its device
// configuration region, which the device sizes. These run the shipping read
// against regions in host memory: the common configuration (for the config
// generation) and a device region of a chosen length.

use nonos_virtio::{Mmio, VirtioError};

use crate::modern::capacity::read;

#[repr(align(4096))]
struct Page([u8; 4096]);

fn regions(page: &mut Page, device_len: usize) -> (Mmio, Mmio) {
    let base = page.0.as_mut_ptr();
    // SAFETY: both ranges lie inside the page, which outlives the test.
    unsafe { (Mmio::new(base, 0x38), Mmio::new(base.add(0x1000 - 0x100), device_len)) }
}

#[test]
fn the_capacity_is_the_little_endian_u64_at_the_start_of_the_device_region() {
    let mut page = Box::new(Page([0; 4096]));
    let at = 0x1000 - 0x100;
    page.0[at..at + 8].copy_from_slice(&0x0000_0012_3456_7800u64.to_le_bytes());
    let (common, device) = regions(&mut page, 0x100);
    assert_eq!(read(&common, device), Ok(0x0000_0012_3456_7800));
}

#[test]
fn a_device_region_too_short_for_the_capacity_is_refused() {
    let mut page = Box::new(Page([0xFF; 4096]));
    for len in 0..8 {
        let (common, device) = regions(&mut page, len);
        assert_eq!(read(&common, device), Err(VirtioError::DeviceCfgShort.message()), "len {len}");
    }
    let (common, device) = regions(&mut page, 8);
    assert_eq!(read(&common, device), Ok(u64::MAX), "exactly eight bytes is enough");
}
