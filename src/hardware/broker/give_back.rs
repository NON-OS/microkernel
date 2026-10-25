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

//! A claim given back with everything granted against it (L66). Retired first,
//! so nothing more is granted and a grant being made is recorded before the
//! drains look; Bus Master stopped, so nothing the device still has in flight
//! lands in a DMA frame freed below; its MMIO, IRQ, DMA and PIO grants torn
//! down; the claim last.

use super::{dma_release_for_device, irq_release_for_device, pio_release_for_device};
use super::{quiesce_held_device, release_device, release_for_device, ClaimError};

/// The released epoch, or why nothing was given back. All of it is bound to
/// the claim the retire found (L69).
pub fn give_back(pid: u32, device_id: u64) -> Result<u64, ClaimError> {
    let epoch = super::claim::retire(pid, device_id)?;
    let _ = quiesce_held_device(pid, device_id, epoch);
    let _ = release_for_device(pid, device_id, epoch);
    let _ = irq_release_for_device(pid, device_id, epoch);
    let _ = dma_release_for_device(pid, device_id, epoch);
    let _ = pio_release_for_device(pid, device_id, epoch);
    release_device(pid, device_id, epoch)
}
