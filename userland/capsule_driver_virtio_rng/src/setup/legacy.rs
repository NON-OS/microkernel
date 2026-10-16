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
//! register window: registers, reset and features, the DMA grants, the
//! ring, Bus Master, DRIVER_OK. A step that fails after the window is
//! mapped resets the device before the caller releases the claim.

use super::driver::Driver;
use super::{dma, pci, registers};
use crate::discover::Found;
use crate::init::{negotiate, start};
use crate::queue::Queue;
use crate::regs::Regs;
use crate::transport::Transport;

pub fn run(dev: Found, claim_epoch: u64) -> Result<Driver, &'static str> {
    let regs = registers::grant(dev, claim_epoch)?;
    let live = live(dev, claim_epoch, regs);
    if live.is_err() {
        let _ = Transport::Legacy(regs).reset();
    }
    live
}

fn live(dev: Found, claim_epoch: u64, regs: Regs) -> Result<Driver, &'static str> {
    negotiate(regs)?;
    let dma = dma::map_both(dev.device_id, claim_epoch)?;
    let master = || pci::command(dev.device_id, claim_epoch, pci::MASTER);
    let queue_size = start(regs, dma.ring.device_addr, master)?;
    Ok(Driver {
        device_id: dev.device_id,
        queue: Queue::new(dma.ring.view, dma.buf.view, dma.buf.device_addr, queue_size),
        transport: Transport::Legacy(regs),
    })
}
