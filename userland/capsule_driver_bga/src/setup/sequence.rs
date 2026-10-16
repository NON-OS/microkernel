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

use nonos_libc::mk_device_release;

use super::{claim, mmio, mode, pci, surface, Driver};
use crate::constants::{BYTES_PER_PIXEL, CLEAR_COLOR, FB_BAR, REG_BAR};
use crate::discover::Found;
use crate::dispi;
use crate::error::{BgaError, BgaResult};
use crate::handles::BrokerHandles;
use crate::regs::Regs;

/// One bring-up attempt on the adapter discovery found. Each failing step
/// releases the claim, which takes any mapping already made with it, so the
/// next attempt can claim the adapter again. Both mappings are as long as
/// the broker made them, not as the mode needs. The adapter must name a
/// DISPI version before anything else is read. The state firmware left is
/// read whole before any write and is written back when the handles drop,
/// on a failed step as when the driver goes. The mode must fit before it is
/// programmed and read back as set, and the surface must be made, before
/// anything is drawn: one refused on the firmware's mode leaves its picture.
pub fn run(dev: Found) -> BgaResult<Driver> {
    let claim_epoch = claim::claim(dev.device_id)?;
    if let Err(e) = pci::enable_memory(dev.device_id, claim_epoch) {
        let _ = mk_device_release(dev.device_id);
        return Err(e);
    }
    let reg_map = mmio::map(dev.device_id, claim_epoch, REG_BAR, dev.reg_size)?;
    let fb_map = mmio::map(dev.device_id, claim_epoch, FB_BAR, dev.fb_size)?;
    let mut handles = BrokerHandles::new(dev.device_id, reg_map.grant_id, fb_map.grant_id);
    if reg_map.length < dispi::regs_len() {
        return Err(BgaError::RegisterWindowTooSmall);
    }
    let regs = Regs::new(reg_map.user_va);
    dispi::identify(regs)?;
    let firmware = dispi::read(regs);
    handles.restore_at_drop(regs, firmware);
    let m = mode::take(regs, &firmware, fb_map.user_va, fb_map.length)?;
    let surface = surface::make(m.width, m.height)?;
    dispi::clear(&m.fb, CLEAR_COLOR);
    let stride = m.width as u32 * BYTES_PER_PIXEL;
    let (width, height, fb) = (m.width, m.height, m.fb);
    Ok(Driver { surface, _handles: handles, regs, fb, width, height, stride, owner_pid: 0 })
}
