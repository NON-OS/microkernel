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

//! DMA phase: two grants. The ring grant holds the descriptor table, the
//! available ring and the used ring; the buffer grant is the page the
//! descriptor points at. A failure leaves the grants made before it in
//! place: the caller resets the device and releases the claim, and the
//! broker stops bus mastering before it tears any of them down. Nothing is
//! unmapped here.

use nonos_libc::{mk_dma_map, DmaMapOut, MK_DMA_MAP_COHERENT};
use nonos_virtio::Mmio;

use crate::constants::{ENTROPY_BUF_LEN, VQ_REGION_SIZE};

#[derive(Clone, Copy)]
pub struct Grant {
    /// The driver's view: every access checked against the length asked for.
    pub view: Mmio,
    /// What the device is told.
    pub device_addr: u64,
}

pub struct Grants {
    pub ring: Grant,
    pub buf: Grant,
}

pub fn map_both(device_id: u64, claim_epoch: u64) -> Result<Grants, &'static str> {
    // The ring is read and written by both sides while it runs: mapped
    // uncached, so neither needs a cache flush (virtio 1.2, 2.7.13).
    let coherent = MK_DMA_MAP_COHERENT;
    Ok(Grants {
        ring: map(device_id, claim_epoch, VQ_REGION_SIZE, coherent, "dma map failed (queue)")?,
        buf: map(device_id, claim_epoch, ENTROPY_BUF_LEN, 0, "dma map failed (buffer)")?,
    })
}

fn map(
    id: u64,
    epoch: u64,
    len: usize,
    flags: u32,
    what: &'static str,
) -> Result<Grant, &'static str> {
    let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    let rc = mk_dma_map(id, epoch, len as u64, flags, &mut out);
    let base = usize::try_from(out.user_va).map_err(|_| what)?;
    let whole = out.length >= len as u64 && out.device_addr.checked_add(len as u64).is_some();
    if rc < 0 || base == 0 || out.device_addr == 0 || !whole {
        return Err(what);
    }
    // SAFETY: MkDmaMap mapped `out.length` bytes at `user_va`, read and
    // write, and `len` of them are used. They stay mapped until the claim is
    // released, and nothing reaches a queue built over them after that.
    let view = unsafe { Mmio::new(base as *mut u8, len) };
    Ok(Grant { view, device_addr: out.device_addr })
}
