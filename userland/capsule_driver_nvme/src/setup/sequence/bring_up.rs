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

use super::after_enable::after_enable;
use crate::admin::{enable, reset_to_disabled, AdminQueue};
use crate::controller::ControllerInfo;
use crate::discover::Found;
use crate::error::{NvmeError, NvmeResult};
use crate::handles::BrokerHandles;
use crate::regs::Regs;
use crate::setup::{claim, irq, mmio, pci, say, Driver};

/// Bring one controller up. Every failure after the claim drops `handles`
/// (and the admin queue before it), which unbinds, unmaps and releases the
/// claim, so the next attempt starts clean; a failure after the enable
/// disables the controller first, so it is not left fetching from queues
/// whose memory is gone.
pub(super) fn bring_up(dev: Found) -> NvmeResult<Driver> {
    let claim_epoch = claim::claim(dev.device_id)?;
    if let Err(e) = pci::enable_device(dev.device_id, claim_epoch) {
        let _ = mk_device_release(dev.device_id);
        return Err(e);
    }
    let mmio = mmio::map(dev.device_id, claim_epoch, dev.bar_size)?;
    let irq = irq::bind(dev, claim_epoch);
    let handles = BrokerHandles::new(dev.device_id, mmio.grant_id, mmio.user_va, irq.grant_id);
    let regs = Regs::new(handles.mmio_user_va());
    let info = ControllerInfo::read(regs);
    if !info.is_nvme_register_block() || !info.doorbells_fit(mmio.length) {
        say::unsupported(&info, mmio.length);
        return Err(NvmeError::UnsupportedController);
    }
    if let Err(e) = reset_to_disabled(regs, info) {
        say::step_failed(b"reset (CC.EN 1 to 0)", e, Some(&info));
        return Err(e);
    }
    let mut admin = AdminQueue::allocate(dev.device_id, claim_epoch)?;
    admin.program_registers(regs);
    if let Err(e) = enable(regs, info) {
        say::step_failed(b"enable (CC.EN 0 to 1)", e, Some(&info));
        let _ = reset_to_disabled(regs, info);
        return Err(e);
    }
    // After the enable, as the reset clears INTMS.
    irq::mask_unbound(regs, irq.grant_id);
    match after_enable(dev, claim_epoch, regs, info, &mut admin) {
        Ok((identity, namespace, health, io, hmb)) => {
            Ok(Driver { _admin: admin, handles, regs, info, identity, namespace, health, io, hmb })
        }
        Err(e) => {
            let _ = reset_to_disabled(regs, info);
            Err(e)
        }
    }
}
