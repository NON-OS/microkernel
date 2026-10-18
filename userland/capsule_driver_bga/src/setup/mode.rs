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

//! The mode the driver shows. Firmware that drew a GOP framebuffer on this
//! adapter (OVMF's QemuVideoDxe) left DISPI programmed for it; that mode is
//! kept, so nothing is re-moded under the firmware's scanout. Otherwise the
//! fixed default is set.

use crate::constants::{MODE_HEIGHT, MODE_WIDTH};
use crate::dispi::{self, Dispi, Span};
use crate::error::BgaResult;
use crate::regs::Regs;

pub struct Mode {
    pub width: u16,
    pub height: u16,
    pub fb: Span,
}

/// The firmware's mode when DISPI holds one this driver draws that fits
/// both the adapter's video memory and the `fb_len` bytes mapped at `fb_va`.
/// Else the default, refused by name before a register is written when it
/// does not fit, and held to reading back as set.
pub fn take(regs: Regs, firmware: &Dispi, fb_va: u64, fb_len: u64) -> BgaResult<Mode> {
    let video = dispi::video_memory(regs);
    if let Some((width, height)) = dispi::kept(firmware) {
        if let Ok(fb) = dispi::fit(video, fb_va, fb_len, width, height) {
            return Ok(Mode { width, height, fb });
        }
    }
    let fb = dispi::fit(video, fb_va, fb_len, MODE_WIDTH, MODE_HEIGHT)?;
    dispi::set_mode(regs, MODE_WIDTH, MODE_HEIGHT);
    dispi::confirm(regs, MODE_WIDTH, MODE_HEIGHT)?;
    Ok(Mode { width: MODE_WIDTH, height: MODE_HEIGHT, fb })
}
