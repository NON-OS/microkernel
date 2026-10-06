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
use super::{claim, dma, irq, modern, registers};
use crate::constants::ENTROPY_BUF_LEN;
use crate::discover::{find_virtio_rng, Found};
use crate::init::bring_up;
use crate::queue::Queue;
use crate::transport::{self, Transport};

/// One bring-up attempt. A failed one holds nothing afterwards.
pub fn run() -> Result<Driver, &'static str> {
    let dev = find_virtio_rng().ok_or("no virtio-rng device")?;

    let claim_epoch = claim::claim(dev.device_id)?;

    let attempt = claimed(dev, claim_epoch);
    if attempt.is_err() {
        /*
         * Whichever step failed, the claim goes, and every MMIO, PIO and
         * DMA grant with it, so the next attempt can claim afresh. Steps
         * that roll back on their own have released it already and this
         * answers "not claimed". A register BAR the driver cannot map and a refused
         * handshake never did, and every attempt after one of them failed
         * at claim.
         */
        let _ = mk_device_release(dev.device_id);
    }
    attempt
}

fn claimed(dev: Found, claim_epoch: u64) -> Result<Driver, &'static str> {
    match transport::probe(&dev, claim_epoch)? {
        Some(caps) => modern::run(dev, claim_epoch, &caps),
        None => legacy(dev, claim_epoch),
    }
}

fn legacy(dev: Found, claim_epoch: u64) -> Result<Driver, &'static str> {
    irq::disable_intx(dev.device_id, claim_epoch)?;

    let register_grant = registers::grant(dev, claim_epoch)?;

    let queue_dma = dma::map_queue(dev.device_id, claim_epoch, register_grant)?;
    let buf_dma = dma::map_buffer(dev.device_id, claim_epoch, register_grant, &queue_dma)?;

    let regs = register_grant.regs().ok_or("virtio-rng: no legacy register window")?;
    let queue_size = bring_up(regs, queue_dma.device_addr, Queue::queue_size())?;
    let queue = Queue::new(
        queue_dma.user_va,
        queue_dma.device_addr,
        buf_dma.user_va,
        buf_dma.device_addr,
        ENTROPY_BUF_LEN as u32,
        queue_size,
    );

    Ok(Driver {
        device_id: dev.device_id,
        claim_epoch,
        register_grant,
        queue_grant: queue_dma.grant_id,
        buf_grant: buf_dma.grant_id,
        queue,
        transport: Transport::Legacy(regs),
    })
}
