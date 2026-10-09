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

//! What one bring-up attempt holds from the broker once every grant is in,
//! and the chip init that either keeps it or gives all of it back.
//!
//! The init used to fail with the claim, the register mapping, the line and
//! the staging DMA all still held; a retry was then refused its own leftover
//! claim, and only the process exit gave them back.

use nonos_libc::{mk_device_release, mk_dma_unmap, mk_irq_unbind, mk_mmio_unmap};

use crate::init::{bring_up, InitState};
use crate::regs::Regs;

#[derive(Clone, Copy)]
pub struct Grants {
    pub device_id: u64,
    pub mmio: u64,
    pub irq: u64,
    pub dma: u64,
}

impl Grants {
    /// Every grant in the reverse of the order it was taken, then the claim,
    /// which on its own tears the rest down as well.
    pub fn release(&self) {
        let _ = mk_dma_unmap(self.dma);
        let _ = mk_irq_unbind(self.irq);
        let _ = mk_mmio_unmap(self.mmio);
        let _ = mk_device_release(self.device_id);
    }
}

/// Wake the chip on an attempt's grants, or give all of them back.
pub fn init_or_release(regs: Regs, grants: &Grants) -> Result<InitState, &'static str> {
    bring_up(regs).inspect_err(|_| grants.release())
}
