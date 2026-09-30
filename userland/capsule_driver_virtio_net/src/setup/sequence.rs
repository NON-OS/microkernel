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

use super::driver::Driver;
use super::stage::stage;
use super::{claim, config, dma_set, irq, queues, registers};
use crate::constants::{LEG_MAC, LEG_QUEUE_NOTIFY, Q_RX, VIRTIO_NET_F_STATUS};
use crate::discover::find_virtio_net;
use crate::init::{driver_ok, negotiate};

pub fn run() -> Result<Driver, &'static str> {
    stage("[net-setup] find");
    let dev = find_virtio_net().ok_or("no virtio-net device")?;
    stage("[net-setup] claim");
    let claim_epoch = claim::claim(dev.device_id)?;
    stage("[net-setup] regmap");
    let register = registers::map(dev, claim_epoch)?;
    stage("[net-setup] irq");
    irq::disable_intx(dev.device_id, claim_epoch, &register)?;
    stage("[net-setup] dma");
    let dma = dma_set::map(dev.device_id, claim_epoch, &register)?;
    let regs = register.regs();
    stage("[net-setup] negotiate");
    let negotiated = negotiate(regs)?;
    let status_supported = config::feature_enabled(negotiated, VIRTIO_NET_F_STATUS);
    // No MSI-X is enabled (nothing is bound), so the legacy device-specific
    // config starts right at the MAC, not four bytes further on.
    let config_base = LEG_MAC;
    let mac = config::read_mac(regs, negotiated, config_base);
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
        regs,
        mac,
        status_supported,
        net_status_offset: config_base + 6,
    })
}
