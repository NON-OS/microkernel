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

//! What the driver reads before it writes: the DISPI ID, the video memory,
//! and the byte order register, set only where the adapter has it and only
//! inside the register mapping.

use nonos_devmodel::FakeBar;

use crate::constants::{DISPI_INDEX_ID, DISPI_INDEX_VIDEO_MEMORY_64K};
use crate::constants::{QEXT_ENDIAN, QEXT_LITTLE_ENDIAN, QEXT_SIZE, REG_BAR_MIN};
use crate::dispi::{dispi_off, identify, little_endian, video_memory};
use crate::error::BgaError;
use crate::regs::Regs;

fn window(index: u32, value: u16) -> FakeBar {
    let bar = FakeBar::new(0x1000);
    bar.present16(dispi_off(index) as usize, value);
    bar
}

#[test]
fn every_dispi_version_is_driven_and_nothing_beside_them() {
    for good in 0xB0C0..=0xB0C5_u16 {
        let bar = window(DISPI_INDEX_ID, good);
        assert_eq!(identify(Regs::new(bar.base())), Ok(()), "{good:#x} refused");
    }
    for bad in [0, 0xFFFF, 0xB0B5, 0xB0C6, 0xB0CF, 0xB0D0, 0x0C0B] {
        let bar = window(DISPI_INDEX_ID, bad);
        assert_eq!(identify(Regs::new(bar.base())), Err(BgaError::NotDispi), "{bad:#x}");
    }
}

#[test]
fn video_memory_is_counted_in_64_kib_units() {
    for (units, bytes) in [(0, 0), (1, 1 << 16), (128, 8 << 20), (u16::MAX, 0xFFFF << 16)] {
        let bar = window(DISPI_INDEX_VIDEO_MEMORY_64K, units);
        assert_eq!(video_memory(Regs::new(bar.base())), bytes, "{units} units");
    }
}

#[test]
fn the_byte_order_is_set_only_where_the_register_is() {
    let cases = [(0, false), (4, false), (7, false), (8, true), (0x200, true), (0x1001, false)];
    for (size, set) in cases {
        let bar = FakeBar::new(0x1000);
        bar.present32(QEXT_SIZE as usize, size);
        assert_eq!(little_endian(Regs::new(bar.base()), 0x1000), set, "size {size:#x}");
        let wrote = bar.wrote32(QEXT_ENDIAN as usize) == QEXT_LITTLE_ENDIAN;
        assert_eq!(wrote, set, "size {size:#x}");
    }
}

#[test]
fn a_register_mapping_the_broker_made_short_of_the_extension_block_is_not_read_there() {
    let bar = FakeBar::new(0x1000);
    bar.present32(QEXT_SIZE as usize, 8);
    for mapped in [0, 0x516, 0x600, REG_BAR_MIN - 1] {
        assert!(!little_endian(Regs::new(bar.base()), mapped), "a {mapped:#x}-byte mapping");
    }
    assert_eq!(bar.wrote32(QEXT_ENDIAN as usize), 0, "written past the mapping");
}
