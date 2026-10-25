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

//! The control queue's DMA grant: the rings and the staging page. A failure
//! leaves the claim to the caller, which releases it; nothing is unmapped.

use nonos_libc::{mk_dma_map, DmaMapOut};
use nonos_virtio::Mmio;

use crate::constants::VQ_REGION_SIZE;

pub struct QueueGrant {
    /// The driver's view: every access checked against the region.
    pub view: Mmio,
    /// What the device is told.
    pub device_addr: u64,
}

pub fn map_queue(device_id: u64, claim_epoch: u64) -> Result<QueueGrant, &'static str> {
    const FAILED: &str = "virtio-gpu: queue dma failed";
    let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    let rc = mk_dma_map(device_id, claim_epoch, VQ_REGION_SIZE as u64, 0, &mut out);
    let base = usize::try_from(out.user_va).map_err(|_| FAILED)?;
    if rc < 0 || base == 0 || out.length < VQ_REGION_SIZE as u64 {
        return Err(FAILED);
    }
    // SAFETY: MkDmaMap mapped `out.length` bytes at `user_va`, read and
    // write, of which VQ_REGION_SIZE are used. They stay mapped until the
    // claim is released, and nothing reaches the queue after that.
    let view = unsafe { Mmio::new(base as *mut u8, VQ_REGION_SIZE) };
    Ok(QueueGrant { view, device_addr: out.device_addr })
}
