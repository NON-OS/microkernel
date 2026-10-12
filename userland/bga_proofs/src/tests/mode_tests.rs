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

//! What the DISPI interface requires of a driver setting a mode.
//!
//! The window is memory and reads back what was written, as the DISPI
//! index block does. `set_mode` writes the mode with the adapter disabled
//! (ENABLE cleared first, set last); a passive window cannot witness that
//! order, so what is asserted is the settled state the adapter acts on.

use nonos_devmodel::FakeBar;

use crate::constants::{
    DISPI_BPP_32, DISPI_ENABLED, DISPI_INDEX_BPP, DISPI_INDEX_ENABLE, DISPI_INDEX_XRES,
    DISPI_INDEX_YRES, DISPI_IOPORT_OFFSET, DISPI_LFB_ENABLED, DISPI_NOCLEARMEM,
};
use crate::dispi::set_mode;
use crate::regs::Regs;

/// Wide enough for the whole DISPI index block.
const WINDOW: usize = 0x600;

fn reg(index: u32) -> usize {
    (DISPI_IOPORT_OFFSET + index * 2) as usize
}

#[test]
fn the_adapter_is_left_with_its_linear_framebuffer_decoding() {
    /*
     * DISPI_LFB_ENABLED is what makes the framebuffer BAR answer at all.
     * Without it every other register reads back correct and the aperture
     * decodes nothing, so the capsule paints into memory the adapter never
     * scans out: a black screen with no failed call behind it.
     */
    let bar = FakeBar::new(WINDOW);
    set_mode(Regs::new(bar.base()), 1024, 768);
    let enable = bar.wrote16(reg(DISPI_INDEX_ENABLE));
    assert!(enable & DISPI_LFB_ENABLED != 0, "the linear framebuffer must be enabled");
    assert!(enable & DISPI_ENABLED != 0, "the adapter must be enabled");
    assert!(enable & DISPI_NOCLEARMEM != 0, "video memory must survive the set");
    assert_eq!(enable & 0x02, 0, "GETCAPS must never be written");
}

#[test]
fn the_mode_is_programmed_at_thirty_two_bits_per_pixel() {
    /*
     * Every primary the wire hands out holds one ARGB8888 u32 per pixel,
     * copied as it is: at 16bpp the screen shows doubled pixels, silently.
     */
    let bar = FakeBar::new(WINDOW);
    set_mode(Regs::new(bar.base()), 1024, 768);
    assert_eq!(bar.wrote16(reg(DISPI_INDEX_BPP)), DISPI_BPP_32);
}

#[test]
fn the_resolution_asked_for_is_the_one_left_in_the_mode_registers() {
    // XRES and YRES are adjacent 16-bit slots: transposed, a portrait mode.
    let bar = FakeBar::new(WINDOW);
    set_mode(Regs::new(bar.base()), 1280, 800);
    assert_eq!(bar.wrote16(reg(DISPI_INDEX_XRES)), 1280);
    assert_eq!(bar.wrote16(reg(DISPI_INDEX_YRES)), 800);
}
