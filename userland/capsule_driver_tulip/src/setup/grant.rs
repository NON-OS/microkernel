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

//! One DMA grant below 4 GiB. Every address the part takes is 32 bits wide,
//! in CSR3 and CSR4 and in each descriptor, so a grant the broker placed
//! higher would be truncated by the part and written somewhere else.

use nonos_libc::{mk_dma_map, DmaMapOut, MK_DMA_MAP_DMA32};

/// The direction bits the broker learns with the lead's L6: the part only
/// reads a transmit buffer and only writes a receive one. A broker without
/// them refuses unknown bits with ENOTSUP, and the same grant is then asked
/// for without them; it is no wider than before, only not yet narrowed.
pub const TO_DEVICE: u32 = 0x10;
pub const FROM_DEVICE: u32 = 0x20;
const ENOTSUP: i64 = -95;
const PAGE_MASK: u64 = 0xFFF;
const DMA32_CEILING: u64 = 1 << 32;

fn map(device_id: u64, epoch: u64, bytes: u64, flags: u32) -> Result<DmaMapOut, i64> {
    let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    let length = (bytes + PAGE_MASK) & !PAGE_MASK;
    let r = mk_dma_map(device_id, epoch, length, flags | MK_DMA_MAP_DMA32, &mut out);
    if r < 0 {
        return Err(r);
    }
    Ok(out)
}

/// The grant, or `None` with nothing left held.
pub fn take(device_id: u64, epoch: u64, bytes: u64, flags: u32, dir: u32) -> Option<DmaMapOut> {
    let out = match map(device_id, epoch, bytes, flags | dir) {
        Err(ENOTSUP) if dir != 0 => map(device_id, epoch, bytes, flags).ok()?,
        r => r.ok()?,
    };
    if fits_32(out.device_addr, out.length) {
        return Some(out);
    }
    let _ = nonos_libc::mk_dma_unmap(out.grant_id);
    None
}

/// Whether `[addr, addr + len)` is nameable in 32 bits.
pub const fn fits_32(addr: u64, len: u64) -> bool {
    match addr.checked_add(len) {
        Some(end) => end <= DMA32_CEILING,
        None => false,
    }
}
