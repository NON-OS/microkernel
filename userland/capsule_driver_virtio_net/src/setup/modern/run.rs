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

//! The modern sequence: PCI command, the register window, the same four
//! DMA grants as the legacy path, then reset, ACKNOWLEDGE, DRIVER, features
//! (VERSION_1 and ACCESS_PLATFORM besides STATUS, never the device's MAC),
//! FEATURES_OK, both queues, a drawn station address, DRIVER_OK. A step that fails returns its error and
//! `sequence::run` releases the claim, which takes every grant with it.

use nonos_virtio::common::{accept, driver_ok, start};
use nonos_virtio::{map_window, ModernCaps, Need, VirtioError};

use super::super::dma_set;
use super::super::driver::Driver;
use super::super::registers::RegisterGrant;
use super::super::stage::stage;
use super::{config, pci, queues};
use crate::constants::{MODERN_WANTED, NET_CFG_STATUS, Q_RX, VIRTIO_NET_F_STATUS};
use crate::discover::Found;
use crate::transport::{LibcBroker, Modern, Transport};


pub fn run(dev: Found, claim_epoch: u64, caps: &ModernCaps) -> Result<Driver, &'static str> {
    stage("[net-setup] modern pci");
    pci::enable(dev.device_id, claim_epoch)?;
    stage("[net-setup] modern regmap");
    let mut broker = LibcBroker::new(dev.device_id, claim_epoch);
    let window = map_window(&mut broker, caps, Need { isr: false, device: true })
        .map_err(VirtioError::message)?;
    let device = window.device.ok_or(VirtioError::NoDeviceCfg.message())?.region;
    let register = RegisterGrant::Modern(window);
    stage("[net-setup] dma");
    let dma = dma_set::map(dev.device_id, claim_epoch, &register)?;
    let common = window.common.region;
    stage("[net-setup] negotiate");
    start(&common).map_err(VirtioError::message)?;
    let features = accept(&common, MODERN_WANTED).map_err(VirtioError::message)?;
    let status_supported = features & (1 << VIRTIO_NET_F_STATUS) != 0;
    // The region must still hold every field the features promise.
    config::read_mac(&common, device, features)?;
    let mac = super::super::station::draw()?;
    stage("[net-setup] queues");
    let q = queues::build(&common, window.notify_area(), &dma)?;
    q.rx.prime();
    driver_ok(&common);
    let transport =
        Transport::Modern(Modern { device, notify: window.notify.region, doorbells: q.doorbells });
    transport.notify(Q_RX);
    stage("[net-setup] done");

    Ok(Driver {
        device_id: dev.device_id,
        register,
        rx_queue_grant: dma.rx_queue.grant_id,
        rx_buffer_grant: dma.rx_buffer.grant_id,
        tx_queue_grant: dma.tx_queue.grant_id,
        tx_buffer_grant: dma.tx_buffer.grant_id,
        rx: q.rx,
        tx: q.tx,
        transport,
        mac,
        status_supported,
        net_status_offset: NET_CFG_STATUS,
    })
}
