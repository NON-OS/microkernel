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

//! DMA phase: three grants, made once the ring's size is known. The ring
//! grant holds the descriptor table and both rings, sized for the ring
//! (`queue::ring_bytes`); the header grant the request header and the
//! status byte; the data grant the most one request moves. A failure
//! leaves the grants made before it in place: the caller resets the device
//! and releases the claim, and the broker stops bus mastering before it
//! tears any of them down. Nothing is unmapped here.

use nonos_libc::{mk_dma_map, DmaMapOut, MK_DMA_MAP_COHERENT};
use nonos_virtio::Mmio;

use crate::constants::{DATA_BUF_LEN, HEADER_BUF_LEN};
use crate::queue::{ring_bytes, Buffer};

const PAGE_MASK: u64 = 0xFFF;

pub struct Grants {
    pub ring: Buffer,
    pub header: Buffer,
    pub data: Buffer,
}

pub fn map_all(id: u64, epoch: u64, queue_size: u16) -> Result<Grants, &'static str> {
    let ring = ring_bytes(queue_size).ok_or("virtio-blk: requestq size")?;
    // The virtqueue is read and written by both sides while it runs: mapped
    // uncached, so neither needs a cache flush (virtio 1.2, 2.7.13).
    let coherent = MK_DMA_MAP_COHERENT;
    Ok(Grants {
        ring: map(id, epoch, ring, coherent, "dma map failed (queue)")?,
        header: map(id, epoch, HEADER_BUF_LEN as usize, 0, "dma map failed (header)")?,
        data: map(id, epoch, DATA_BUF_LEN as usize, 0, "dma map failed (data)")?,
    })
}

/// A grant of at least `len` bytes, page aligned on both sides (the legacy
/// ring is found by page number), at addresses whose ends do not wrap.
fn map(
    id: u64,
    epoch: u64,
    len: usize,
    flags: u32,
    what: &'static str,
) -> Result<Buffer, &'static str> {
    let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    let rc = mk_dma_map(id, epoch, len as u64, flags, &mut out);
    let base = usize::try_from(out.user_va).map_err(|_| what)?;
    let whole = out.length >= len as u64 && out.device_addr.checked_add(len as u64).is_some();
    let aligned = (out.user_va | out.device_addr) & PAGE_MASK == 0;
    if rc < 0 || base == 0 || out.device_addr == 0 || !whole || !aligned {
        return Err(what);
    }
    // SAFETY: MkDmaMap mapped `out.length` bytes at `user_va`, read and
    // write, and `len` of them are used. They stay mapped until the claim is
    // released, and nothing reaches a queue built over them after that.
    let view = unsafe { Mmio::new(base as *mut u8, len) };
    Ok(Buffer { view, phys: out.device_addr })
}
