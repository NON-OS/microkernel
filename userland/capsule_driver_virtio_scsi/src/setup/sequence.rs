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

//! The bring-up: claim, memory decode and bus mastering, the capability
//! walk, the common, notify and device regions, the DMA grant, the
//! handshake and the request queue; then the targets scanned. Nothing is
//! bound: the driver polls, and the available ring asks for no interrupt.

use nonos_libc::mk_device_claim;
use nonos_libc::mk_device_release;
use nonos_scsi_serve::Units;
use nonos_virtio::{map_window, parse, ConfigSpace, Need, VirtioError};

use super::driver::{abandon, Driver};
use super::{broker::LibcBroker, dma, handshake, pci};
use crate::discover::Found;
use crate::hba::VirtioHba;
use crate::{NAME, REPLY_ENDPOINT};

pub fn run(dev: Found) -> Result<Driver, &'static str> {
    let epoch = mk_device_claim(dev.device_id);
    if epoch < 0 {
        return Err("claim failed");
    }
    let epoch = epoch as u64;
    let release = |e: &'static str| {
        let _ = mk_device_release(dev.device_id);
        e
    };
    pci::enable(dev.device_id, epoch).map_err(release)?;
    let mut broker = LibcBroker::new(dev.device_id, epoch);
    let cfg = ConfigSpace::read(&mut broker).ok_or(VirtioError::ConfigRead.message());
    let caps = parse(&cfg.map_err(release)?, &dev.bars);
    let need = Need { isr: false, device: true };
    let window =
        map_window(&mut broker, &caps, need).map_err(VirtioError::message).map_err(release)?;
    let dma = dma::map(dev.device_id, epoch).inspect_err(|e| {
        abandon(dev.device_id, epoch, &window, None);
    })?;
    let fail = |e: &'static str| {
        abandon(dev.device_id, epoch, &window, Some(dma.grant_id));
        e
    };
    let common = window.common.region;
    let device = window.device.ok_or(VirtioError::NoDeviceCfg.message()).map_err(fail)?.region;
    let notify = window.notify.region;
    let area = window.notify_area();
    let limits = handshake::run(&common, &device, area, dma.device_addr).map_err(fail)?;
    let hba = VirtioHba::new(common, device, notify, area, limits, dma.user_va, dma.device_addr);
    let mut units = Units::new(hba, REPLY_ENDPOINT, NAME);
    units.scan(limits.max_target, limits.max_lun);
    Ok(Driver { units, window, device_id: dev.device_id, epoch, dma_grant: dma.grant_id })
}
