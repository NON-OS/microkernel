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

use crate::controller::ControllerInfo;
use crate::discover::Found;
use crate::error::{NvmeError, NvmeResult};
use crate::handles::{release, BrokerHandles};
use crate::regs::Regs;
use crate::setup::{claim, irq, mmio, pci, say, Device};

/// Take a controller and stop it before it may master the bus; the device
/// and its claim epoch. A predecessor that died without its teardown can
/// leave the controller enabled, its queues and host memory buffer naming
/// frames the kernel has freed and handed on since, and the claim does not
/// reset it: with Bus Master on it would go on writing them until RDY reads
/// 0. So the registers are reached with Memory Space alone, the controller
/// is disabled and seen stopped, and only then is Bus Master written; no
/// refusal on the way writes it. Each failure lets the device go one way:
/// once its registers read as NVMe's, `device` is dropped, which releases
/// the claim, the release clearing Bus Master before it drains every grant;
/// before that, `handles` or `release` lets it go.
pub fn stopped(dev: Found) -> NvmeResult<(Device, u64)> {
    let claim_epoch = claim::claim(dev.device_id)?;
    if let Err(e) = pci::decode(dev.device_id, claim_epoch) {
        release(dev.device_id, claim_epoch);
        return Err(e);
    }
    let mmio = mmio::map(dev.device_id, claim_epoch, dev.bar_size)?;
    let irq = irq::bind(dev, claim_epoch);
    let handles =
        BrokerHandles::new(dev.device_id, claim_epoch, mmio.grant_id, mmio.user_va, irq.grant_id);
    let regs = Regs::new(handles.mmio_user_va());
    let info = ControllerInfo::read(regs);
    if !info.is_nvme_register_block() || !info.doorbells_fit(mmio.length) {
        say::unsupported(&info, mmio.length);
        return Err(NvmeError::UnsupportedController);
    }
    let mut device = Device::new(handles, regs, info);
    if let Err(e) = device.disable() {
        say::step_failed(b"reset (CC.EN 1 to 0)", e, Some(&info));
        return Err(e);
    }
    pci::bus_master(dev.device_id, claim_epoch)?;
    Ok((device, claim_epoch))
}
