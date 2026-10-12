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

//! One DMA grant, mapped and checked, as a bounds-checked region.

use nonos_libc::{mk_dma_map, DmaMapOut};
use nonos_virtio::Mmio;

use super::grant::usable;

#[derive(Clone, Copy)]
pub struct Grant {
    /// The driver's view: every access checked against the length asked for.
    pub view: Mmio,
    /// What the device is told.
    pub device_addr: u64,
}

/// Map `len` bytes for the claimed device. A failure leaves the grants made
/// before it in place: the caller releases the claim, and the broker stops
/// bus mastering before it tears any of them down.
pub fn map(
    device_id: u64,
    claim_epoch: u64,
    len: usize,
    flags: u32,
    what: &'static str,
) -> Result<Grant, &'static str> {
    let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    let rc = mk_dma_map(device_id, claim_epoch, len as u64, flags, &mut out);
    if rc < 0 || !usable(out.user_va, out.device_addr, out.length, len) {
        return Err(what);
    }
    let base = usize::try_from(out.user_va).map_err(|_| what)? as *mut u8;
    // SAFETY: MkDmaMap mapped `out.length` bytes at `user_va`, read and
    // write, and `usable` checked they cover `len`. They stay mapped until
    // the claim is released, and nothing reaches a queue built over them
    // after that: teardown is the last step before a fresh bring-up.
    let view = unsafe { Mmio::new(base, len) };
    Ok(Grant { view, device_addr: out.device_addr })
}
