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

//! Which mode is kept: the firmware's whenever it can be drawn as it is,
//! so the adapter is not re-moded under it; else the driver sets its own
//! (`room_tests`).

use crate::constants::DISPI_LFB_ENABLED;
use crate::constants::{DISPI_BPP_32, DISPI_ENABLED, DISPI_INDEX_BANK, DISPI_INDEX_BPP};
use crate::constants::{DISPI_INDEX_ENABLE, DISPI_INDEX_VIRT_WIDTH, DISPI_INDEX_XRES};
use crate::constants::{DISPI_INDEX_X_OFFSET, DISPI_INDEX_YRES, DISPI_INDEX_Y_OFFSET};
use crate::dispi::{current, dispi_off, DispiMode};
use crate::model::Adapter;

const ON: u16 = DISPI_ENABLED | DISPI_LFB_ENABLED;
/// The mode OVMF leaves for a 1280x800 GOP framebuffer, as read back.
pub const OVMF: DispiMode = DispiMode { width: 1280, height: 800, stride: 1280 * 4 };

/// DISPI as OVMF leaves it for that framebuffer, then `poke` written over it.
pub fn ovmf((index, value): (u32, u16)) -> Adapter {
    let a = Adapter::new();
    let (bpp, virt) = (DISPI_INDEX_BPP, DISPI_INDEX_VIRT_WIDTH);
    let mode = [(DISPI_INDEX_XRES, 1280), (DISPI_INDEX_YRES, 800), (bpp, DISPI_BPP_32)];
    for (i, v) in mode.into_iter().chain([(DISPI_INDEX_ENABLE, ON), (virt, 1280), (index, value)]) {
        a.bar.present16(dispi_off(i) as usize, v);
    }
    a
}

#[test]
fn the_firmwares_mode_is_kept_with_its_other_enable_bits_and_its_bank() {
    // ENABLE's other bits, the 8-bit DAC among them, and the bank are left.
    for poke in [(DISPI_INDEX_BANK, 0), (DISPI_INDEX_BANK, 3), (DISPI_INDEX_ENABLE, ON | 0x20)] {
        assert_eq!(current(ovmf(poke).regs()), Some(OVMF), "{poke:?}");
    }
}

#[test]
fn a_mode_this_driver_cannot_draw_as_it_is_is_not_kept() {
    // Lines wider than the mode are drawn at their stride (`current_tests`);
    // narrower ones overlap, and a copy at any stride would shear them.
    let cases = [
        (DISPI_INDEX_ENABLE, DISPI_ENABLED),
        (DISPI_INDEX_ENABLE, DISPI_LFB_ENABLED),
        (DISPI_INDEX_ENABLE, 0),
        (DISPI_INDEX_BPP, 16),
        (DISPI_INDEX_VIRT_WIDTH, 1024),
        (DISPI_INDEX_VIRT_WIDTH, 0),
        (DISPI_INDEX_X_OFFSET, 8),
        (DISPI_INDEX_Y_OFFSET, 800),
        (DISPI_INDEX_XRES, 0),
        (DISPI_INDEX_YRES, 0),
    ];
    for poke in cases {
        assert_eq!(current(ovmf(poke).regs()), None, "{poke:?} kept");
    }
}
