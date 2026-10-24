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

//! The DISPI state firmware left, written back when the handles drop: on a
//! failed attempt as when the driver goes.

use super::dispi_off::dispi_off;
use super::state::{read, Dispi};
use crate::constants::{
    DISPI_INDEX_BANK, DISPI_INDEX_BPP, DISPI_INDEX_ENABLE, DISPI_INDEX_VIRT_WIDTH,
    DISPI_INDEX_XRES, DISPI_INDEX_X_OFFSET, DISPI_INDEX_YRES, DISPI_INDEX_Y_OFFSET,
};
use crate::regs::Regs;

/// Write `d` back, unless the adapter still holds it: a write-back disables
/// and re-enables the adapter, which on QEMU clears video memory, so an
/// attempt refused on the firmware's own mode leaves its picture showing.
pub fn restore(regs: Regs, d: &Dispi) {
    // SAFETY: every index written lies below `regs_len`, as for `read`.
    write_back(&read(regs), d, |index, value| unsafe { regs.w16(dispi_off(index), value) });
}

/// The register writes, `w(index, value)`, that put `d` back over `now`:
/// none when `now` is `d`. Else the mode with the adapter disabled, as
/// `set_mode` does, then the virtual width and the offsets after ENABLE,
/// since an adapter enabled from disabled resets them (as observed of
/// QEMU's adapter, unverified).
pub fn write_back(now: &Dispi, d: &Dispi, mut w: impl FnMut(u32, u16)) {
    if now == d {
        return;
    }
    w(DISPI_INDEX_ENABLE, 0);
    w(DISPI_INDEX_BANK, d.bank);
    w(DISPI_INDEX_XRES, d.xres);
    w(DISPI_INDEX_YRES, d.yres);
    w(DISPI_INDEX_BPP, d.bpp);
    w(DISPI_INDEX_ENABLE, d.enable);
    w(DISPI_INDEX_VIRT_WIDTH, d.virt_width);
    w(DISPI_INDEX_X_OFFSET, d.x_offset);
    w(DISPI_INDEX_Y_OFFSET, d.y_offset);
}
