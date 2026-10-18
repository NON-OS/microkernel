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

//! The mode firmware left showing. OVMF programs DISPI for its GOP
//! framebuffer, and a driver that set its own default re-moded the adapter
//! under the firmware's scanout. A mode the driver can draw is kept, with no
//! mode register written; one it cannot is replaced by the default.

use super::left::showing;
use crate::adapter::{adapter, found, Adapter, MODE_BYTES, REG_LEN};
use crate::constants::{CLEAR_COLOR, DISPI_ENABLED, DISPI_INDEX_BPP, DISPI_INDEX_ENABLE};
use crate::constants::{DISPI_INDEX_VIDEO_MEMORY_64K, DISPI_INDEX_VIRT_WIDTH, DISPI_INDEX_XRES};
use crate::constants::{DISPI_INDEX_X_OFFSET, DISPI_INDEX_YRES, DISPI_INDEX_Y_OFFSET};
use crate::dispi::dispi_off;
use crate::setup::run;

/// What firmware left, over 800x600; video memory in 64 KiB; the mode driven.
type Case = (&'static str, fn(&Adapter), u16, (u16, u16));

fn set(a: &Adapter, index: u32, value: u16) {
    a.regs.present16(dispi_off(index) as usize, value);
}

#[test]
fn the_mode_firmware_left_showing_is_kept_without_a_write() {
    let a = adapter(REG_LEN, MODE_BYTES);
    showing(&a, 800, 600);
    // The 8-bit DAC bit: the driver's own mode set would clear it.
    set(&a, DISPI_INDEX_ENABLE, 0x61);
    let d = run(found(MODE_BYTES)).unwrap_or_else(|e| panic!("refused: {e:?}"));
    assert_eq!((d.width, d.height, d.stride), (800, 600, 3200), "the firmware's mode replaced");
    let reg = |index| a.regs.wrote16(dispi_off(index) as usize);
    assert_eq!((reg(DISPI_INDEX_XRES), reg(DISPI_INDEX_ENABLE)), (800, 0x61), "re-moded");
    assert_eq!(a.fb.wrote32((800 * 600 - 1) * 4), CLEAR_COLOR, "the kept mode was not cleared");
    assert_eq!(a.fb.wrote32(800 * 600 * 4), 0, "cleared past the kept mode");
}

#[test]
fn only_a_mode_the_driver_can_draw_is_kept() {
    let no_lfb = |a: &Adapter| set(a, DISPI_INDEX_ENABLE, DISPI_ENABLED);
    let cases: [Case; 10] = [
        ("as OVMF leaves it", |_| {}, 256, (800, 600)),
        ("VGA text, DISPI off", |a| set(a, DISPI_INDEX_ENABLE, 0), 256, (1024, 768)),
        ("no linear framebuffer", no_lfb, 256, (1024, 768)),
        ("16 bits a pixel", |a| set(a, DISPI_INDEX_BPP, 16), 256, (1024, 768)),
        ("lines wider than the mode", |a| set(a, DISPI_INDEX_VIRT_WIDTH, 1024), 256, (1024, 768)),
        ("panned across", |a| set(a, DISPI_INDEX_X_OFFSET, 8), 256, (1024, 768)),
        ("panned down", |a| set(a, DISPI_INDEX_Y_OFFSET, 1), 256, (1024, 768)),
        ("no height", |a| set(a, DISPI_INDEX_YRES, 0), 256, (1024, 768)),
        ("past video memory", |a| showing(a, 1280, 1024), 64, (1024, 768)),
        ("past BAR0", |a| showing(a, 1280, 800), 256, (1024, 768)),
    ];
    for (name, left, video, want) in cases {
        let a = adapter(REG_LEN, MODE_BYTES);
        showing(&a, 800, 600);
        left(&a);
        set(&a, DISPI_INDEX_VIDEO_MEMORY_64K, video);
        let d = run(found(MODE_BYTES)).unwrap_or_else(|e| panic!("{name}: refused: {e:?}"));
        assert_eq!((d.width, d.height), want, "{name}");
        assert_eq!(a.regs.wrote16(dispi_off(DISPI_INDEX_XRES) as usize), want.0, "{name}");
    }
}
