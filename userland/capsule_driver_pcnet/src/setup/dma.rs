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

//! DMA phase: the control page (init block and both rings, coherent), the
//! receive pool and the transmit pool. Each step gives back every earlier
//! grant on failure so the broker never holds a partial setup.

use nonos_libc::{DmaMapOut, MmioMapOut, MK_DMA_MAP_COHERENT};

use super::grant::{take, FROM_DEVICE, TO_DEVICE};
use super::rollback;
use crate::constants::layout::CONTROL_BYTES;
use crate::constants::queue::{RX_POOL_BYTES, TX_POOL_BYTES};

pub struct Grants {
    pub control: DmaMapOut,
    pub rx_pool: DmaMapOut,
    pub tx_pool: DmaMapOut,
}

pub fn map_all(device_id: u64, epoch: u64, mmio: &MmioMapOut) -> Result<Grants, &'static str> {
    let control =
        take(device_id, epoch, CONTROL_BYTES as u64, MK_DMA_MAP_COHERENT, 0).ok_or_else(|| {
            rollback::after(device_id, mmio, &[]);
            "dma map failed (control page)"
        })?;
    let rx_pool =
        take(device_id, epoch, RX_POOL_BYTES as u64, 0, FROM_DEVICE).ok_or_else(|| {
            rollback::after(device_id, mmio, &[control.grant_id]);
            "dma map failed (rx pool)"
        })?;
    let tx_pool = take(device_id, epoch, TX_POOL_BYTES as u64, 0, TO_DEVICE).ok_or_else(|| {
        rollback::after(device_id, mmio, &[control.grant_id, rx_pool.grant_id]);
        "dma map failed (tx pool)"
    })?;
    Ok(Grants { control, rx_pool, tx_pool })
}
