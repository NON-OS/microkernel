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

//! One bring-up attempt on the adapter discovery found. Bus mastering
//! stays off: the adapter reads nothing of the host's (section 12). Each
//! failing step releases the claim, which takes any mapping already made
//! with it, so the next attempt can claim the adapter again.

use nonos_gfx_pool::Pool;
use nonos_gfx_serve::Server;
use nonos_libc::mk_device_release;

use super::{claim, mmio, mode, Driver};
use crate::constants::{EDID_WINDOW, FB_BAR, MK_MMIO_MAP_WC, REG_BAR};
use crate::discover::Found;
use crate::display::{fits, Bga};
use crate::edid::{self, Panel};
use crate::error::{BgaError, BgaResult};
use crate::handles::BrokerHandles;
use crate::regs::Regs;

pub fn run(dev: Found) -> BgaResult<Driver> {
    let epoch = claim::claim(dev.device_id)?;
    let reg_map = mmio::map(dev.device_id, epoch, REG_BAR, 0, dev.reg_size)?;
    let fb_map = mmio::map(dev.device_id, epoch, FB_BAR, MK_MMIO_MAP_WC, dev.fb_size)?;
    let handles = BrokerHandles::new(dev.device_id, reg_map.grant_id, fb_map.grant_id);
    let regs = Regs::new(reg_map.user_va);
    let vram_len = fb_map.length as usize;
    let mut blob = [0u8; EDID_WINDOW];
    let edid_len = if reg_map.length as usize >= EDID_WINDOW {
        // SAFETY: the register window maps at least the EDID window.
        unsafe { edid::read(reg_map.user_va, &mut blob) }
    } else {
        0
    };
    let panel = Panel::read(&blob[..edid_len], |w, h| fits(vram_len, w, h));
    let Some(mode) = mode::starting(regs, &panel, vram_len) else {
        drop(handles);
        let _ = mk_device_release(dev.device_id);
        return Err(BgaError::NoMode);
    };
    let bga = Bga { regs, vram: fb_map.user_va, vram_len, panel, mode, generation: 1, fence };
    Ok(Driver { handles: handles, server: Server::new(bga, Pool) })
}

fn fence() {
    // SAFETY: sfence has no operands and orders the WC stores before it.
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::x86_64::_mm_sfence()
    };
}
