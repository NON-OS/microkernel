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

//! One DMA grant. vmxnet3 takes 64-bit addresses everywhere, so the broker
//! may place it anywhere.

use nonos_libc::{mk_dma_map, DmaMapOut};

/// The direction bits the broker learns with the maintainers' L6: the device only
/// reads a transmit buffer and only writes a receive one. A broker without
/// them refuses unknown bits with ENOTSUP, and the same grant is then asked
/// for without them; it is no wider than before, only not yet narrowed.
pub const TO_DEVICE: u32 = 0x10;
pub const FROM_DEVICE: u32 = 0x20;
const ENOTSUP: i64 = -95;
const PAGE_MASK: u64 = 0xFFF;

fn map(device_id: u64, epoch: u64, bytes: u64, flags: u32) -> Result<DmaMapOut, i64> {
    let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    let length = (bytes + PAGE_MASK) & !PAGE_MASK;
    let r = mk_dma_map(device_id, epoch, length, flags, &mut out);
    if r < 0 {
        return Err(r);
    }
    Ok(out)
}

/// The grant, or `None` with nothing held.
pub fn take(device_id: u64, epoch: u64, bytes: u64, flags: u32, dir: u32) -> Option<DmaMapOut> {
    match map(device_id, epoch, bytes, flags | dir) {
        Err(ENOTSUP) if dir != 0 => map(device_id, epoch, bytes, flags).ok(),
        r => r.ok(),
    }
}
