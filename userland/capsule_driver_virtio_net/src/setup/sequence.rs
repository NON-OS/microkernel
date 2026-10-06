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

use super::driver::Driver;
use super::stage::stage;
use super::{claim, config, dma_set, irq, modern, queues, registers};
use crate::constants::{LEG_QUEUE_NOTIFY, NET_CFG_STATUS, Q_RX, VIRTIO_NET_F_STATUS};
use crate::discover::{find_virtio_net, Found};
use crate::init::{driver_ok, negotiate};
use crate::transport::{self, Transport};

/// One bring-up attempt. A failed one holds nothing afterwards.
pub fn run() -> Result<Driver, &'static str> {
    stage("[net-setup] find");
    let dev = find_virtio_net().ok_or("no virtio-net device")?;
    stage("[net-setup] claim");
    let claim_epoch = claim::claim(dev.device_id)?;
    let attempt = claimed(dev, claim_epoch);
    if attempt.is_err() {
        /*
         * Whichever step failed, the claim goes, and every MMIO, PIO and
         * DMA grant with it, so the next attempt can claim afresh. Steps
         * that roll back on their own have released it already and this
         * answers "not claimed". Negotiation and queue programming never
         * did, and every attempt after one of them failed at claim.
         */
        let _ = mk_device_release(dev.device_id);
    }
    attempt
}

fn claimed(dev: Found, claim_epoch: u64) -> Result<Driver, &'static str> {
    stage("[net-setup] transport");
    match transport::probe(&dev, claim_epoch)? {
        Some(caps) => modern::run(dev, claim_epoch, &caps),
        None => legacy(dev, claim_epoch),
    }
}

fn legacy(dev: Found, claim_epoch: u64) -> Result<Driver, &'static str> {
    stage("[net-setup] regmap");
    let register = registers::map(dev, claim_epoch)?;
    stage("[net-setup] irq");
    irq::disable_intx(dev.device_id, claim_epoch, &register)?;
    stage("[net-setup] dma");
    let dma = dma_set::map(dev.device_id, claim_epoch, &register)?;
    let regs = register.regs().ok_or("virtio-net: no legacy register window")?;
    stage("[net-setup] negotiate");
    let negotiated = negotiate(regs)?;
    let status_supported = config::feature_enabled(negotiated, VIRTIO_NET_F_STATUS);
    // The station transmits from a drawn address; the device's is never read.
    let mac = super::station::draw()?;
    stage("[net-setup] queues");
    let (rx, tx) = queues::build(regs, &dma)?;
    rx.prime();
    driver_ok(regs);
    unsafe {
        regs.w16(LEG_QUEUE_NOTIFY, Q_RX);
    }
    stage("[net-setup] done");

    Ok(Driver {
        device_id: dev.device_id,
        register,
        rx_queue_grant: dma.rx_queue.grant_id,
        rx_buffer_grant: dma.rx_buffer.grant_id,
        tx_queue_grant: dma.tx_queue.grant_id,
        tx_buffer_grant: dma.tx_buffer.grant_id,
        rx,
        tx,
        transport: Transport::Legacy(regs),
        mac,
        status_supported,
        net_status_offset: NET_CFG_STATUS,
    })
}
