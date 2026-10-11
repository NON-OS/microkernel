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

use super::surface::Shadow;
use super::{claim, mmio, pci, Driver};
use crate::constants::{CLEAR_COLOR, FB_BAR, REG_BAR};
use crate::discover::Found;
use crate::dispi::{self, Choice};
use crate::error::{BgaError, BgaResult};
use crate::gfx::Screen;
use crate::regs::Regs;

/// One bring-up attempt on the adapter discovery found. A failing step drops
/// the claim, which releases the adapter and its grants, so the next attempt
/// can claim it again.
pub fn run(dev: Found) -> BgaResult<Driver> {
    let claim = claim::claim(dev.device_id)?;
    pci::enable_memory(claim.device_id(), claim.epoch)?;
    let reg = mmio::map(claim.device_id(), claim.epoch, REG_BAR, dev.reg_size)?;
    // SAFETY: the broker mapped `reg_size` bytes, and discovery required at
    // least REG_BAR_MIN; the claim held in the Driver keeps the map live.
    let regs = unsafe { Regs::new(reg.user_va) };
    dispi::id(&regs).ok_or(BgaError::NotDispi)?;
    dispi::little_endian(&regs, dev.reg_size);
    let vram = dispi::vram(&regs, dev.fb_size);
    let choice = dispi::choose(dispi::seen(&regs), vram).ok_or(BgaError::NoMode)?;
    let mode = match choice {
        Choice::Keep(m) | Choice::Set(m) => m,
    };
    let fb_len = mode.bytes().next_multiple_of(4096).min(dev.fb_size);
    let fb = mmio::map(claim.device_id(), claim.epoch, FB_BAR, fb_len)?;
    if let Choice::Set(m) = choice {
        if !dispi::set_mode(&regs, m) {
            return Err(BgaError::ModeRefused);
        }
        let pixels = m.bytes() / 4;
        // SAFETY: `fb` maps `mode.bytes()` bytes, `pixels * 4` of them.
        unsafe { dispi::clear(fb.user_va, pixels, CLEAR_COLOR) };
    }
    let shadow = Shadow::new(mode)?;
    Ok(Driver { shadow, _claim: claim, fb_va: fb.user_va, mode, screen: Screen::new(mode) })
}
