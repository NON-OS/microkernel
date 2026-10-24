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

//! One bring-up attempt on the adapter discovery found. Memory decoding
//! goes on and bus mastering stays off: the adapter reads nothing of the
//! host's (section 12). Each failing step releases the claim, which takes
//! any mapping already made with it, so the next attempt can claim the
//! adapter again. Both mappings are as long as the broker made them. The
//! adapter must name a DISPI version before anything else is read; the
//! state firmware left is read whole before any write and written back
//! when the handles drop, on a failed step as when the driver goes; then
//! its framebuffer is set little endian where it has the register.

use nonos_gfx_serve::{Platform, Server};
use nonos_libc::{mk_device_release, MK_MMIO_MAP_WC};

use super::fresh::fresh;
use super::{claim, mmio, pci, Driver};
use crate::constants::{EDID_WINDOW, FB_BAR, REG_BAR};
use crate::discover::Found;
use crate::error::{BgaError, BgaResult};
use crate::handles::BrokerHandles;
use crate::regs::Regs;
use crate::{dispi, edid};

/// The primaries go on `platform`.
pub fn run<P: Platform>(dev: Found, platform: P) -> BgaResult<Driver<P>> {
    let epoch = claim::claim(dev.device_id)?;
    if let Err(e) = pci::enable_memory(dev.device_id, epoch) {
        let _ = mk_device_release(dev.device_id);
        return Err(e);
    }
    let reg_map = mmio::map(dev.device_id, epoch, REG_BAR, 0, dev.reg_size)?;
    let fb_map = mmio::map(dev.device_id, epoch, FB_BAR, MK_MMIO_MAP_WC, dev.fb_size)?;
    let mut handles = BrokerHandles::new(dev.device_id, reg_map.grant_id, fb_map.grant_id);
    if reg_map.length < dispi::regs_len() {
        return Err(BgaError::RegisterWindowTooSmall);
    }
    let regs = Regs::new(reg_map.user_va);
    dispi::identify(regs)?;
    handles.restore_at_drop(regs, dispi::read(regs));
    // Not written back at drop: little endian is the only order drawn in.
    dispi::little_endian(regs, reg_map.length);
    let mut blob = [0u8; EDID_WINDOW];
    let edid_len = if reg_map.length as usize >= EDID_WINDOW {
        // SAFETY: the register window maps at least the EDID window.
        unsafe { edid::read(reg_map.user_va, &mut blob) }
    } else {
        0
    };
    let bga = fresh(regs, (fb_map.user_va, fb_map.length), &blob[..edid_len], fence)?;
    Ok(Driver { handles, server: Server::new(bga, platform) })
}

fn fence() {
    // SAFETY: sfence has no operands and orders the WC stores before it.
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::x86_64::_mm_sfence()
    };
}
