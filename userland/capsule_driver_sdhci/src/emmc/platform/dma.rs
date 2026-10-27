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

//! A broker DMA region. Nothing here unmaps one: `mk_device_release` frees
//! every grant the device holds once the broker has read Bus Master back
//! off, and an unmap before that would free a frame the host could still
//! reach, since the driver's own "off" write changes nothing until the
//! broker stores Command words as written.

use nonos_libc::{mk_dma_map, DmaMapOut, MK_DMA_MAP_DMA32};

use super::super::env::DmaBuf;
use super::super::error::{EmmcError, EmmcResult};

/// What a grant shorter than asked counts as: the region asked for was not
/// given, as when the broker has no memory for it (ENOMEM).
const SHORT_GRANT: i64 = -12;

/// A zeroed, physically contiguous region of `length` bytes (a multiple of
/// 4 KiB) the claimed device may reach until the claim is released. For a
/// host without 64-bit addressing (`dma64` false) it is asked below 4 GiB:
/// ADMA2 in 32-bit mode names nothing above, and the kernel may place a map
/// that does not ask there. A grant shorter than `length` is refused, and
/// one longer is used for `length` bytes only, so the ADMA2 table and every
/// transfer are bounded by what was asked, never by what came back.
pub fn map(device_id: u64, claim_epoch: u64, length: u64, dma64: bool) -> EmmcResult<DmaBuf> {
    let flags = if dma64 { 0 } else { MK_DMA_MAP_DMA32 };
    let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    let r = mk_dma_map(device_id, claim_epoch, length, flags, &mut out);
    if r < 0 {
        return Err(EmmcError::Broker(r));
    }
    if out.length < length {
        return Err(EmmcError::Broker(SHORT_GRANT));
    }
    Ok(DmaBuf { va: out.user_va, bus: out.device_addr, len: length as usize })
}
