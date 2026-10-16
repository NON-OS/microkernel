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

//! The modern handshake over a mapped window: the interrupt (MSI-X first),
//! reset (waited for by the clock), ACKNOWLEDGE, DRIVER, features
//! (VERSION_1, ACCESS_PLATFORM whenever offered, FLUSH when offered,
//! nothing else), FEATURES_OK, the ring size, the DMA grants, Bus Master,
//! the request queue, the capacity, DRIVER_OK.

use nonos_libc::mk_irq_ack;
use nonos_virtio::common::{accept, driver_ok, queue_max, start_until};
use nonos_virtio::{VirtioError, Window};

use super::super::driver::Driver;
use super::super::step::step;
use super::super::{dma, irq, pci};
use super::{capacity, queue};
use crate::clock::expired_after;
use crate::constants::{MODERN_QUEUE_SIZE, RESET_MS};
use crate::discover::Found;
use crate::queue::fit;
use crate::transport::{Modern, Transport};

/// VIRTIO_BLK_F_FLUSH. DISCARD, WRITE_ZEROES, MQ and the rest are never
/// taken (plan/57 names them their own unit).
const VIRTIO_BLK_F_FLUSH: u64 = 1 << 9;

pub(super) fn live(dev: Found, epoch: u64, window: &Window) -> Result<Driver, &'static str> {
    let common = window.common.region;
    let isr = window.isr.ok_or(VirtioError::NoIsrCfg.message())?.region;
    let device = window.device.ok_or(VirtioError::NoDeviceCfg.message())?.region;
    step("irq-bind");
    let irq = irq::bind(&dev, epoch)?;
    step("negotiate");
    start_until(&common, expired_after(RESET_MS)).map_err(VirtioError::message)?;
    accept(&common, VIRTIO_BLK_F_FLUSH).map_err(VirtioError::message)?;
    let size = fit(queue_max(&common, 0), MODERN_QUEUE_SIZE).ok_or("virtio-blk: requestq size")?;
    step("dma");
    let dma = dma::map_all(dev.device_id, epoch, size)?;
    pci::command(dev.device_id, epoch, pci::MASTER)?;
    step("bring-up");
    let ready = queue::setup(&common, window.notify_area(), &dma, size, irq.msix)?;
    let capacity_sectors = capacity::read(&common, device)?;
    if capacity_sectors == 0 {
        return Err("virtio-blk: zero capacity");
    }
    driver_ok(&common);
    if mk_irq_ack(irq.grant) < 0 {
        return Err("virtio-blk: irq ack failed");
    }
    step("ready");
    let (notify, doorbell) = (window.notify.region, ready.doorbell);
    let transport = Transport::Modern(Modern { common, notify, doorbell, isr });
    let (device_id, irq_grant, queue) = (dev.device_id, irq.grant, ready.queue);
    Ok(Driver { device_id, claim_epoch: epoch, irq_grant, queue, transport, capacity_sectors })
}
