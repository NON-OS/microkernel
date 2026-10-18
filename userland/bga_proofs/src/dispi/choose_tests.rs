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

//! Which mode is driven: the firmware's whenever it can be drawn as it is,
//! so the boot's screen stays up, else the driver's own; each held to the
//! video memory and to the BAR0 mapping it must fit, to the byte.

use crate::constants::{DISPI_BPP_32, DISPI_ENABLED, DISPI_LFB_ENABLED, MODE_HEIGHT, MODE_WIDTH};
use crate::dispi::{fit, kept, Dispi};
use crate::error::BgaError;

const ON: u16 = DISPI_ENABLED | DISPI_LFB_ENABLED;
const MIB: u64 = 1 << 20;
/// DISPI as OVMF leaves it for a 1280x800 GOP framebuffer.
const OVMF: Dispi =
    Dispi { xres: 1280, yres: 800, bpp: DISPI_BPP_32, enable: ON, virt_width: 1280, ..NONE };
const NONE: Dispi =
    Dispi { xres: 0, yres: 0, bpp: 0, enable: 0, bank: 0, virt_width: 0, x_offset: 0, y_offset: 0 };

#[test]
fn the_firmwares_mode_is_kept_so_the_boot_screen_stays_up() {
    assert_eq!(kept(&OVMF), Some((1280, 800)));
    // ENABLE's other bits, the 8-bit DAC among them, and the bank are left.
    assert_eq!(kept(&Dispi { enable: ON | 0x20, bank: 3, ..OVMF }), Some((1280, 800)));
}

#[test]
fn a_mode_this_driver_cannot_draw_as_it_is_is_not_kept() {
    let cases = [
        Dispi { enable: DISPI_ENABLED, ..OVMF },
        Dispi { enable: DISPI_LFB_ENABLED, ..OVMF },
        Dispi { enable: 0, ..OVMF },
        Dispi { bpp: 16, ..OVMF },
        Dispi { virt_width: 1024, ..OVMF },
        Dispi { virt_width: 1536, ..OVMF },
        Dispi { x_offset: 8, ..OVMF },
        Dispi { y_offset: 800, ..OVMF },
        Dispi { xres: 0, ..OVMF },
        Dispi { yres: 0, ..OVMF },
        NONE,
    ];
    for d in cases {
        assert_eq!(kept(&d), None, "{d:?} kept");
    }
}

#[test]
fn video_memory_and_the_bar0_mapping_each_bound_both_modes_to_the_byte() {
    // 1280x800x4 is 4,096,000 bytes; 1024x768x4 is 3 MiB.
    let fits = |video, w, h| fit(video, 0x1000, 16 * MIB, w, h).map(|_| ());
    assert_eq!(fits(4_096_000, 1280, 800), Ok(()));
    assert_eq!(fits(4_095_999, 1280, 800), Err(BgaError::VideoMemoryTooSmall));
    assert_eq!(fits(3 * MIB, MODE_WIDTH, MODE_HEIGHT), Ok(()));
    assert_eq!(fits(3 * MIB - 1, MODE_WIDTH, MODE_HEIGHT), Err(BgaError::VideoMemoryTooSmall));
    // However much video memory there is, the mapping bounds the mode too.
    let mapped = |len, w, h| fit(64 * MIB, 0x1000, len, w, h).map(|_| ());
    assert_eq!(mapped(4_096_000, 1280, 800), Ok(()));
    assert_eq!(mapped(4_095_999, 1280, 800), Err(BgaError::ModeTooLarge));
    assert_eq!(mapped(3 * MIB, MODE_WIDTH, MODE_HEIGHT), Ok(()));
    assert_eq!(mapped(3 * MIB - 1, MODE_WIDTH, MODE_HEIGHT), Err(BgaError::ModeTooLarge));
}
