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
use super::{claim, dma, irq, modern, registers};
use crate::constants::LEG_CFG_CAPACITY;
use crate::discover::{find_virtio_blk, Found};
use crate::init::bring_up;
use crate::queue::Queue;
use crate::transport::{self, Transport};
use nonos_libc::{mk_debug, mk_device_release, mk_irq_ack};

const MSIX_CONFIG_SHIFT: usize = 4;

// The step about to run, printed before it. A step that returns an error is
// reported by the bring-up loop in main once it gives up; a step that never
// returns reports nothing there, and the last line printed here names it.
pub(super) fn step(name: &str) {
    let mut line = [0u8; 64];
    let tag = b"[BLK] step ";
    let n = tag.len();
    line[..n].copy_from_slice(tag);
    let m = name.len().min(line.len() - n);
    line[n..n + m].copy_from_slice(&name.as_bytes()[..m]);
    let _ = mk_debug(line.as_ptr(), n + m);
}

/// One bring-up attempt. A failed one holds nothing afterwards.
pub fn run() -> Result<Driver, &'static str> {
    step("find");
    let dev = find_virtio_blk().ok_or("no virtio-blk device")?;
    step("claim");
    let claim_epoch = claim::claim(dev.device_id)?;
    let attempt = claimed(dev, claim_epoch);
    if attempt.is_err() {
        /*
         * Whichever step failed, the claim goes, and every MMIO, PIO, IRQ
         * and DMA grant with it, so the next attempt can claim afresh. The
         * steps below roll back on their own and this then answers "not
         * claimed"; a register BAR the driver cannot map never did, and a modern
         * handshake that fails leaves the release to this line.
         */
        let _ = mk_device_release(dev.device_id);
    }
    attempt
}

fn claimed(dev: Found, claim_epoch: u64) -> Result<Driver, &'static str> {
    step("transport");
    match transport::probe(&dev, claim_epoch)? {
        Some(caps) => modern::run(dev, claim_epoch, &caps),
        None => legacy(dev, claim_epoch),
    }
}

fn legacy(dev: Found, claim_epoch: u64) -> Result<Driver, &'static str> {
    step("regs");
    let register_grant = registers::grant(dev, claim_epoch)?;
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
    let regs = register_grant.regs().ok_or("virtio-blk: no legacy register window")?;
    step("bring-up");
    let init = match bring_up(regs, queue_dma.device_addr, Queue::max_supported_size(), msix) {
        Ok(init) => init,
        Err(e) => {
            dma::rollback::data(
                dev.device_id,
                register_grant,
                &irq_grant,
                &queue_dma,
                &header_dma,
                &data_dma,
            )?;
            return Err(e);
        }
    };
    let queue = Queue::new(
        queue_dma.user_va,
        queue_dma.device_addr,
        init.queue_size,
        header_dma.user_va,
        header_dma.device_addr,
        data_dma.user_va,
        data_dma.device_addr,
    );
    let capacity_offset =
        if msix { LEG_CFG_CAPACITY + MSIX_CONFIG_SHIFT } else { LEG_CFG_CAPACITY };
    let capacity_sectors = unsafe { regs.r64(capacity_offset) };
    if capacity_sectors == 0 {
        dma::rollback::data(
            dev.device_id,
            register_grant,
            &irq_grant,
            &queue_dma,
            &header_dma,
            &data_dma,
        )?;
        return Err("virtio-blk: zero capacity");
    }
    if mk_irq_ack(irq_grant.grant_id) < 0 {
        dma::rollback::data(
            dev.device_id,
            register_grant,
            &irq_grant,
            &queue_dma,
            &header_dma,
            &data_dma,
        )?;
        return Err("virtio-blk: irq ack failed");
    }
    step("ready");
    Ok(Driver {
        irq_grant: irq_grant.grant_id,
        queue,
        transport: Transport::Legacy(regs),
        capacity_sectors,
    })
}
