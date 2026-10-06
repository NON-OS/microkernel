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

//! DMA buffer mapping. Cap requirement: `Dma`.

use super::types::DmaMapOut;
use crate::syscall::{call_raw, N_MK_DMA_MAP, N_MK_DMA_UNMAP};

pub const MK_DMA_MAP_HIGH: u32 = 1 << 0;
/// The device takes only 32-bit DMA addresses (an AHCI HBA without
/// CAP.S64A, for one). The broker hands out frames below 4 GiB or fails
/// the map; it never returns an address the device would truncate. Not
/// together with `MK_DMA_MAP_HIGH`. Without either flag the broker still
/// prefers memory below 4 GiB, but falls back to any.
/// A map whose device address (the IOVA, when a remapping unit confines the
/// device) would still end above 4 GiB fails with -34 (ERANGE).
pub const MK_DMA_MAP_DMA32: u32 = 1 << 1;
/// Map the buffer uncached, for a descriptor ring the device and the driver
/// both write while it runs: no cache flush is needed on either side, as
/// Linux dma_alloc_coherent. Not together with `MK_DMA_MAP_WC`.
pub const MK_DMA_MAP_COHERENT: u32 = 1 << 2;
/// Map the buffer write-combining (uncached where the CPU has no such PAT
/// entry), for data the driver writes whole before it rings the device.
/// Stores may merge and reorder: fence (sfence) before the doorbell.
pub const MK_DMA_MAP_WC: u32 = 1 << 3;

#[no_mangle]
pub extern "C" fn mk_dma_map(
    device_id: u64,
    claim_epoch: u64,
    length: u64,
    flags: u32,
    out: *mut DmaMapOut,
) -> i64 {
    call_raw(N_MK_DMA_MAP, [device_id, claim_epoch, length, flags as u64, out as u64, 0])
}

#[no_mangle]
pub extern "C" fn mk_dma_unmap(grant_id: u64) -> i64 {
    call_raw(N_MK_DMA_UNMAP, [grant_id, 0, 0, 0, 0, 0])
}
