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

//! The MAC from struct virtio_net_config in the device region.

use nonos_virtio::common::stable_read;
use nonos_virtio::{Mmio, VirtioError};

use crate::constants::{MAC_LEN, NET_CFG_STATUS, VIRTIO_NET_F_MAC, VIRTIO_NET_F_STATUS};

/// The MAC when the device offered one, all zeros otherwise (as on the
/// legacy path). The region must hold every field the features promise.
pub fn read_mac(common: &Mmio, device: Mmio, features: u64) -> Result<[u8; MAC_LEN], &'static str> {
    let has = |bit: u32| features & (1u64 << bit) != 0;
    let needed = if has(VIRTIO_NET_F_STATUS) { NET_CFG_STATUS + 2 } else { MAC_LEN };
    if device.len() < needed {
        return Err(VirtioError::DeviceCfgShort.message());
    }
    if !has(VIRTIO_NET_F_MAC) {
        return Ok([0u8; MAC_LEN]);
    }
    // Six single-byte reads; the generation check keeps them one MAC.
    stable_read(common, || core::array::from_fn(|i| device.r8(i)))
        .ok_or(VirtioError::DeviceCfgUnstable.message())
}
