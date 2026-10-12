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

//! The legacy bring-up, for a transitional function with its legacy register
//! window: registers, the interrupt pin off, reset and features, a drawn
//! station address, the DMA grants, Bus Master, both queues, DRIVER_OK. A
//! step that fails returns its error and `sequence::run` releases the claim.

use super::driver::Driver;
use super::stage::stage;
use super::{config, dma_set, irq, pci, queues, registers, station};
use crate::constants::{NET_CFG_STATUS, Q_RX, VIRTIO_NET_F_STATUS};
use crate::discover::Found;
use crate::init::{driver_ok, negotiate};
use crate::transport::Transport;

pub fn run(dev: Found, claim_epoch: u64) -> Result<Driver, &'static str> {
    stage("[net-setup] regmap");
    let register = registers::map(dev, claim_epoch)?;
    stage("[net-setup] irq");
    irq::disable_intx(dev.device_id, claim_epoch, &register)?;
    let regs = register.regs().ok_or("virtio-net: no legacy register window")?;
    stage("[net-setup] negotiate");
    let negotiated = negotiate(regs)?;
    // The station transmits from a drawn address; the device's is never read.
    let mac = station::draw()?;
    stage("[net-setup] dma");
    let dma = dma_set::map_all(dev.device_id, claim_epoch)?;
    pci::command(dev.device_id, claim_epoch, pci::MASTER)?;
    stage("[net-setup] queues");
    let (mut rx, tx) = queues::build(regs, &dma)?;
    rx.prime();
    driver_ok(regs);
    let transport = Transport::Legacy(regs);
    transport.notify(Q_RX);
    stage("[net-setup] done");
    Ok(Driver {
        device_id: dev.device_id,
        rx,
        tx,
        transport,
        mac,
        status_supported: config::feature_enabled(negotiated, VIRTIO_NET_F_STATUS),
        net_status_offset: NET_CFG_STATUS,
    })
}
