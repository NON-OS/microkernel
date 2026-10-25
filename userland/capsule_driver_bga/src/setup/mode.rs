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

//! The mode bga starts on: firmware's when DISPI already shows a linear
//! 32-bit mode from the start of video memory, its lines no narrower than
//! it, that fits (section 12: a driver keeps the mode firmware left until a
//! MODE_SET, so OVMF's GOP mode stays set with no re-mode; the kernel's GOP
//! blit is gone by then, stopped by the claim of an adapter holding the
//! boot framebuffer); else the monitor's preferred mode when it fits, else
//! 1024x768, refused by name before a register is written when it does not
//! fit either limit, and held to reading back as set.

use crate::constants::FALLBACK_MODE;
use crate::dispi::{current, set_mode, DispiMode};
use crate::display::fits;
use crate::edid::Panel;
use crate::error::{BgaError, BgaResult};
use crate::regs::Regs;

/// `video` bytes of video memory, `mapped` bytes of BAR0 mapped.
pub fn starting(regs: Regs, panel: &Panel, (video, mapped): (u64, u64)) -> BgaResult<DispiMode> {
    let room = video.min(mapped) as usize;
    if let Some(m) = current(regs).filter(|m| m.stride as usize * m.height as usize <= room) {
        return Ok(m);
    }
    let (w, h) = panel.preferred.filter(|&(w, h)| fits(room, w, h)).unwrap_or(FALLBACK_MODE);
    let bytes = w as u64 * h as u64 * 4;
    if bytes > video {
        return Err(BgaError::VideoMemoryTooSmall);
    }
    if bytes > mapped {
        return Err(BgaError::ModeTooLarge);
    }
    set_mode(regs, w as u16, h as u16);
    current(regs).filter(|m| (m.width, m.height) == (w, h)).ok_or(BgaError::ModeNotTaken)
}
