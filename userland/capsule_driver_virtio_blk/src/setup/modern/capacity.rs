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

//! The capacity from struct virtio_blk_config in the device region.

use nonos_virtio::common::stable_read;
use nonos_virtio::{Mmio, VirtioError};

/// Bytes of struct virtio_blk_config the driver reads: the 64-bit capacity.
const CAPACITY_LEN: usize = 8;

/// Capacity in 512-byte sectors, read as two 32-bit halves under the config
/// generation so a resize between them cannot pair two different values.
pub fn read(common: &Mmio, device: Mmio) -> Result<u64, &'static str> {
    if device.len() < CAPACITY_LEN {
        return Err(VirtioError::DeviceCfgShort.message());
    }
    stable_read(common, || ((device.r32(4) as u64) << 32) | device.r32(0) as u64)
        .ok_or(VirtioError::DeviceCfgUnstable.message())
}
