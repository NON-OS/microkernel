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

//! What the capsule holds for its life: the units it serves over the IOC,
//! and what it gives back if it leaves.

use nonos_libc::{mk_device_release, mk_dma_unmap, mk_mmio_unmap};

use crate::constants::regs::{
    DOORBELL_FUNCTION_SHIFT, FUNCTION_IOC_MESSAGE_UNIT_RESET, REG_DOORBELL,
};
use crate::hba::MptHba;
use crate::regs::{Mmio, Regs};
use crate::Units;

pub struct Driver {
    pub units: Units<MptHba<Mmio>>,
    pub device_id: u64,
    pub mmio_grant: u64,
    pub dma_grant: u64,
}

impl Driver {
    /// A message unit reset, so the IOC stops touching the frames and the
    /// buffers, then the window, the DMA grant and the claim given back.
    pub fn release(&self) {
        abandon(&self.units.hba.regs, self.device_id, self.mmio_grant, Some(self.dma_grant));
    }
}

/// The same, for a bring-up that fails after the window is mapped.
pub fn abandon(regs: &Mmio, device_id: u64, mmio_grant: u64, dma_grant: Option<u64>) {
    regs.w32(REG_DOORBELL, (FUNCTION_IOC_MESSAGE_UNIT_RESET as u32) << DOORBELL_FUNCTION_SHIFT);
    let _ = mk_mmio_unmap(mmio_grant);
    if let Some(g) = dma_grant {
        let _ = mk_dma_unmap(g);
    }
    let _ = mk_device_release(device_id);
}
