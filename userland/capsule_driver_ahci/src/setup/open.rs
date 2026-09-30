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

use super::{claim, irq, mmio, pci};
use crate::constants::MAX_PORTS;
use crate::controller::{enable_ahci, scan_ports, settle_links, ControllerInfo, PortInfo};
use crate::discover::Found;
use crate::error::AhciResult;
use crate::handles::BrokerHandles;
use crate::regs::Regs;

/// A controller the driver claimed, mapped and put in AHCI mode. Dropping it
/// drops `handles`, which unbinds the interrupt, unmaps ABAR and releases the
/// claim; the broker clears bus mastering on release.
pub(super) struct Opened {
    pub epoch: u64,
    pub handles: BrokerHandles,
    pub regs: Regs,
    pub info: ControllerInfo,
    pub ports: [PortInfo; MAX_PORTS],
}

pub(super) fn open(dev: Found) -> AhciResult<Opened> {
    let epoch = claim::claim(dev.device_id)?;
    if let Err(e) = pci::enable_bus_master(dev.device_id, epoch) {
        let _ = mk_device_release(dev.device_id);
        return Err(e);
    }
    let mmio = mmio::map(dev.device_id, epoch, dev.abar_size)?;
    let irq = irq::bind(dev, epoch);

    let handles = BrokerHandles::new(dev.device_id, mmio.grant_id, mmio.user_va, irq.grant_id);
    let regs = Regs::new(handles.mmio_user_va());

    enable_ahci(regs);
    let info = ControllerInfo::read(regs);
    settle_links(regs, info.pi, info.port_count);
    let ports = scan_ports(regs, info.pi, info.port_count);
    Ok(Opened { epoch, handles, regs, info, ports })
}
