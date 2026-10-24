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

use super::master::master_after_reset;
use super::remap::say_remapped;
use super::say_hba::say_hba;
use super::say_port_map::say_port_map;
use super::{claim, irq, mmio, pci};
use crate::constants::MAX_PORTS;
use crate::controller::ports::effective_pi;
use crate::controller::{
    enable_ahci, ports_in_window, scan_ports, settle_links, spin_up, ControllerInfo, PortInfo,
};
use crate::discover::Found;
use crate::error::AhciResult;
use crate::handles::BrokerHandles;
use crate::regs::Regs;

/// A controller the driver claimed, mapped, reset into AHCI mode and gave Bus
/// Master once its ports read stopped. Dropping it drops `handles`, which
/// writes Bus Master off, unbinds the interrupt, unmaps ABAR and releases
/// the claim (`BrokerHandles`'s `Drop`); the release reads Bus Master off.
pub(super) struct Opened {
    pub epoch: u64,
    pub handles: BrokerHandles,
    pub regs: Regs,
    pub info: ControllerInfo,
    pub ports: [PortInfo; MAX_PORTS],
}

pub(super) fn open(dev: Found) -> AhciResult<Opened> {
    let epoch = claim::claim(dev.device_id)?;
    // Memory Space alone: Bus Master waits for the HBA reset (`master`).
    if let Err(e) = pci::command(dev.device_id, epoch, false) {
        let _ = mk_device_release(dev.device_id);
        return Err(e);
    }
    let mmio = mmio::map(dev.device_id, epoch, dev.abar_size)?;
    let irq = irq::bind(dev, epoch);
    let (id, va, irq) = (mmio.grant_id, mmio.user_va, irq.grant_id);
    let handles = BrokerHandles::new(dev.device_id, epoch, id, va, irq);
    // The broker's VA is that of ABAR's first byte, inside its page when the
    // ABAR does not start on one.
    let regs = Regs::new(handles.mmio_user_va());

    enable_ahci(regs)?;
    let mut info = ControllerInfo::read(regs);
    // Every later port walk goes by this PI, so no port past the mapped
    // window is ever read or written.
    let raw_pi = info.pi;
    info.pi = effective_pi(info.cap, raw_pi) & ports_in_window(mmio.length);
    say_hba(&info, raw_pi, mmio.length);
    say_port_map(dev.vendor, dev.device_id, epoch);
    master_after_reset(dev.device_id, epoch, regs, info.pi, info.port_count)?;
    say_remapped(regs, dev.abar_size, mmio.length);
    spin_up(regs, info.cap, info.pi, info.port_count);
    settle_links(regs, info.pi, info.port_count);
    let ports = scan_ports(regs, info.pi, info.port_count);
    Ok(Opened { epoch, handles, regs, info, ports })
}
