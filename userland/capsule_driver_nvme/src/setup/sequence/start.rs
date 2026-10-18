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

use super::after_enable::after_enable;
use crate::admin::AdminQueue;
use crate::discover::Found;
use crate::error::NvmeResult;
use crate::setup::{say, Device, Driver};

/// The rest of bringing one controller up, once it is stopped with Bus
/// Master on (bring_up::stopped): the admin queue, the enable, and what is
/// read once enabled. A failure drops `device`, which disables the
/// controller and then releases it; nothing the controller was given is
/// unmapped first, so the next attempt starts clean and no frame is reused
/// under a live device.
pub(super) fn start(dev: Found, mut device: Device, claim_epoch: u64) -> NvmeResult<Driver> {
    let (regs, info) = (device.regs, device.info);
    let mut admin = AdminQueue::allocate(dev.device_id, claim_epoch)?;
    admin.program_registers(regs);
    if let Err(e) = device.enable() {
        say::step_failed(b"enable (CC.EN 0 to 1)", e, Some(&info));
        return Err(e);
    }
    let (identity, namespace, health, mut io, hmb) =
        after_enable(dev, claim_epoch, regs, info, &mut admin)?;
    if let Some(io) = io.as_mut() {
        io.park_on(device.handles.irq_grant_id());
    }
    Ok(Driver { device, admin, identity, namespace, health, io, hmb })
}
