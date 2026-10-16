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

//! Giving the device back, from any attempt that took DMA frames: both
//! engines stopped and the part reset first, then Bus Master turned off,
//! then the device released. The release frees every grant tied to the
//! device after the broker has read Bus Master back off, so no frame is
//! unmapped here: the receiver writes wire frames into its buffer for as
//! long as RE is set, and a buffer freed under it is someone else's memory.
//! There is no mk_dma_unmap or mk_pio_release call here:
//! `mk_device_release` does both, in the kernel, once Bus Master reads back
//! off.

use nonos_libc::{mk_device_release, PioGrantOut};

use super::pci::command;
use crate::constants::regs::REG_CMD;
use crate::init::reset_part;
use crate::pio::Pio;

pub fn after_pio(device_id: u64, epoch: u64, pio: &PioGrantOut) {
    release(device_id, epoch, &Pio::new(pio.grant_id));
}

/// Best-effort throughout: a part that will not stop or reset still goes
/// back, with Bus Master off, and the broker's own stop runs before any
/// frame is freed.
pub fn release(device_id: u64, epoch: u64, pio: &Pio) {
    let _ = pio.w8(REG_CMD, 0);
    let _ = reset_part(pio);
    let _ = command(device_id, epoch, false);
    let _ = mk_device_release(device_id);
}
