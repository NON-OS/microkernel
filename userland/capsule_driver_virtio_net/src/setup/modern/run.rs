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

//! The modern sequence: memory decode, the register window, reset (waited
//! for by the clock), ACKNOWLEDGE, DRIVER, features (VERSION_1 and
//! ACCESS_PLATFORM besides STATUS, never the device's MAC), FEATURES_OK, a
//! drawn station address, the DMA grants, both queues, Bus Master,
//! DRIVER_OK. A step that fails returns its error and `sequence::run`
//! releases the claim, which takes every grant with it.

use nonos_virtio::common::{accept, driver_ok, start_until};
use nonos_virtio::{map_window, ModernCaps, Need, VirtioError};

use super::super::driver::Driver;
use super::super::stage::stage;
use super::super::{dma_set, pci, station};
use super::{config, queues};
use crate::clock::expired_after;
use crate::constants::{MODERN_WANTED, NET_CFG_STATUS, Q_RX, RESET_MS, VIRTIO_NET_F_STATUS};
use crate::discover::Found;
use crate::transport::{LibcBroker, Modern, Transport};

pub fn run(dev: Found, claim_epoch: u64, caps: &ModernCaps) -> Result<Driver, &'static str> {
    stage("[net-setup] modern pci");
    pci::command(dev.device_id, claim_epoch, pci::DECODE)?;
    stage("[net-setup] modern regmap");
    let mut broker = LibcBroker::new(dev.device_id, claim_epoch);
    let window = map_window(&mut broker, caps, Need { isr: false, device: true })
        .map_err(VirtioError::message)?;
    let device = window.device.ok_or(VirtioError::NoDeviceCfg.message())?.region;
    let common = window.common.region;
    stage("[net-setup] negotiate");
    start_until(&common, expired_after(RESET_MS)).map_err(VirtioError::message)?;
    let features = accept(&common, MODERN_WANTED).map_err(VirtioError::message)?;
    // The region must still hold every field the features promise.
    config::read_mac(&common, device, features)?;
    let mac = station::draw()?;
    stage("[net-setup] dma");
    let dma = dma_set::map_all(dev.device_id, claim_epoch)?;
    stage("[net-setup] queues");
    let master = || pci::command(dev.device_id, claim_epoch, pci::MASTER);
    let mut q = queues::build(&common, window.notify_area(), &dma, master)?;
    q.rx.prime();
    driver_ok(&common);
    let notify = window.notify.region;
    let transport = Transport::Modern(Modern { common, device, notify, doorbells: q.doorbells });
    transport.notify(Q_RX);
    stage("[net-setup] done");
    Ok(Driver {
        device_id: dev.device_id,
        rx: q.rx,
        tx: q.tx,
        transport,
        mac,
        status_supported: features & (1 << VIRTIO_NET_F_STATUS) != 0,
        net_status_offset: NET_CFG_STATUS,
    })
}
