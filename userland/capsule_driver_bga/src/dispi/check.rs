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

//! What the adapter says back: the DISPI interface version before a mode
//! is set, and the mode it holds after. The adapter takes any write to its
//! mode registers without failing; one that cannot show a mode clamps or
//! drops it, so only reading it back tells.

use super::dispi_off::dispi_off;
use super::state::{read, Dispi};
use crate::constants::{
    DISPI_BPP_32, DISPI_ENABLED, DISPI_ID_FIRST, DISPI_ID_LAST, DISPI_INDEX_ID, DISPI_LFB_ENABLED,
};
use crate::error::{BgaError, BgaResult};
use crate::regs::Regs;

/// Whether the ID register names a DISPI version this driver programs.
/// Anything else behind this PCI ID, or a window that reads all ones, is
/// not driven.
pub fn identify(regs: Regs) -> BgaResult<()> {
    // SAFETY: the ID register lies below `regs_len`, which the caller has
    // checked the register mapping covers.
    let id = unsafe { regs.r16(dispi_off(DISPI_INDEX_ID)) };
    if (DISPI_ID_FIRST..=DISPI_ID_LAST).contains(&id) {
        Ok(())
    } else {
        Err(BgaError::NotDispi)
    }
}

/// Whether the adapter holds the `width` x `height` mode at 32 bits per
/// pixel, enabled with its linear framebuffer, scanned out from the start of
/// video memory one mode-wide line after another: the layout every copy into
/// the framebuffer writes.
pub fn confirm(regs: Regs, width: u16, height: u16) -> BgaResult<()> {
    if holds(&read(regs), width, height) {
        Ok(())
    } else {
        Err(BgaError::ModeNotTaken)
    }
}

/// The mode firmware left showing, when it is one this driver can draw:
/// OVMF programs DISPI for its GOP framebuffer, and a kept mode keeps the
/// boot's picture up with no re-mode. The kernel's GOP blit is gone by
/// then: the claim of an adapter holding the boot framebuffer stopped it.
pub fn kept(d: &Dispi) -> Option<(u16, u16)> {
    (d.xres != 0 && d.yres != 0 && holds(d, d.xres, d.yres)).then_some((d.xres, d.yres))
}

fn holds(d: &Dispi, width: u16, height: u16) -> bool {
    let on = DISPI_ENABLED | DISPI_LFB_ENABLED;
    let mode = (d.xres, d.yres, d.bpp, d.enable & on);
    mode == (width, height, DISPI_BPP_32, on)
        && (d.virt_width, d.x_offset, d.y_offset) == (width, 0, 0)
}
