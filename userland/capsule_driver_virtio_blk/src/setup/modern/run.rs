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

//! The modern sequence: PCI command, the common, notify, ISR and device
//! regions, the interrupt (INTx, else one MSI-X vector) and the three DMA
//! grants exactly as on the legacy path, then reset, ACKNOWLEDGE, DRIVER,
//! features (VERSION_1, ACCESS_PLATFORM when offered, FLUSH when offered),
//! FEATURES_OK, the request queue, the capacity, DRIVER_OK. A step that
//! fails returns its error and `sequence::run` releases the claim, which
//! takes every grant with it.

use nonos_libc::mk_irq_ack;
use nonos_virtio::common::driver_ok;
use nonos_virtio::{map_window, ModernCaps, Need, VirtioError};

use super::super::driver::Driver;
use super::super::registers::RegisterGrant;
use super::super::sequence::step;
use super::super::{dma, irq};
use super::handshake::{self, Dma};
use super::{capacity, pci};
use crate::discover::Found;
use crate::transport::{LibcBroker, Modern, Transport};

pub fn run(dev: Found, claim_epoch: u64, caps: &ModernCaps) -> Result<Driver, &'static str> {
    step("modern-pci");
    pci::enable(dev.device_id, claim_epoch)?;
    step("modern-regs");
    let mut broker = LibcBroker::new(dev.device_id, claim_epoch);
    let window = map_window(&mut broker, caps, Need { isr: true, device: true })
        .map_err(VirtioError::message)?;
    let isr = window.isr.ok_or(VirtioError::NoIsrCfg.message())?.region;
    let device = window.device.ok_or(VirtioError::NoDeviceCfg.message())?.region;
    let register_grant = RegisterGrant::Modern(window);
    step("irq-bind");
    let (irq_grant, msix) = irq::bind(dev, claim_epoch, register_grant)?;
    step("dma-queue");
    let queue_dma = dma::map_queue(dev.device_id, claim_epoch, register_grant, &irq_grant)?;
    step("dma-header");
    let header_dma =
        dma::map_header(dev.device_id, claim_epoch, register_grant, &irq_grant, &queue_dma)?;
    step("dma-data");
    let data_dma = dma::map_data(
        dev.device_id,
        claim_epoch,
        register_grant,
        &irq_grant,
        &queue_dma,
        &header_dma,
    )?;
    step("bring-up");
    let common = window.common.region;
    let dma = Dma { queue: &queue_dma, header: &header_dma, data: &data_dma };
    let ready = handshake::run(&common, window.notify_area(), dma, msix)?;
    let capacity_sectors = capacity::read(&common, device)?;
    if capacity_sectors == 0 {
        return Err("virtio-blk: zero capacity");
    }
    driver_ok(&common);
    if mk_irq_ack(irq_grant.grant_id) < 0 {
        return Err("virtio-blk: irq ack failed");
    }
    step("ready");
    let transport =
        Transport::Modern(Modern { notify: window.notify.region, doorbell: ready.doorbell, isr });
    Ok(Driver { irq_grant: irq_grant.grant_id, queue: ready.queue, transport, capacity_sectors })
}
