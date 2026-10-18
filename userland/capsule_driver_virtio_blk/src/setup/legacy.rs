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

//! The legacy bring-up, for a function that presents only its legacy
//! register window: registers, the interrupt (MSI-X first), reset and
//! features, the ring size the device dictates, the DMA grants sized for
//! it, Bus Master, the ring, the capacity, DRIVER_OK. A step that fails
//! once the window is mapped resets the device before the caller releases
//! the claim: the device has the ring's page number from `program` on, and
//! was left pointing at frames the release then freed.

use nonos_libc::mk_irq_ack;

use super::driver::Driver;
use super::step::step;
use super::{dma, irq, pci, registers};
use crate::discover::Found;
use crate::init::{capacity, driver_ok, negotiate, program, queue_size};
use crate::queue::Queue;
use crate::regs::Regs;
use crate::transport::Transport;

pub fn run(dev: Found, claim_epoch: u64) -> Result<Driver, &'static str> {
    step("regs");
    let regs = registers::grant(dev, claim_epoch)?;
    let live = live(dev, claim_epoch, regs);
    if live.is_err() {
        let _ = Transport::Legacy(regs).reset();
    }
    live
}

fn live(dev: Found, claim_epoch: u64, regs: Regs) -> Result<Driver, &'static str> {
    step("irq-bind");
    let irq = irq::bind(&dev, claim_epoch)?;
    step("negotiate");
    negotiate(regs)?;
    let size = queue_size(regs)?;
    step("dma");
    let dma = dma::map_all(dev.device_id, claim_epoch, size)?;
    let queue = Queue::new(dma.ring.view, size, dma.header, dma.data).ok_or("virtio-blk: ring")?;
    let queue = queue.on_legacy();
    pci::command(dev.device_id, claim_epoch, pci::MASTER)?;
    step("bring-up");
    program(regs, dma.ring.phys, irq.msix)?;
    let capacity_sectors = capacity(regs, irq.msix)?;
    driver_ok(regs);
    if mk_irq_ack(irq.grant) < 0 {
        return Err("virtio-blk: irq ack failed");
    }
    step("ready");
    Ok(Driver {
        device_id: dev.device_id,
        claim_epoch,
        irq_grant: irq.grant,
        queue,
        transport: Transport::Legacy(regs),
        capacity_sectors,
    })
}
