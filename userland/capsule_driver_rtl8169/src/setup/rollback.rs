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

//! Giving the device back. Once DMA frames exist, both engines are stopped
//! and the part reset first, then Bus Master turned off, then the device
//! released. The release frees every grant tied to the device after the
//! broker has read Bus Master back off, so no frame is unmapped here: a part
//! left receiving could still write wire frames into a buffer freed under it,
//! and read whatever now sits in a freed ring as descriptors. There is no
//! mk_dma_unmap or mk_mmio_unmap call here: `mk_device_release` does both,
//! in the kernel, once Bus Master reads back off.

use nonos_libc::{mk_device_release, MmioMapOut};

use super::pci::command;
use crate::chip::MacVersion;
use crate::constants::regs::REG_CMD;
use crate::init::reset_part;
use crate::regs::Regs;

/// Before any DMA frame is taken: Bus Master off, then the device back.
pub fn unclaim(device_id: u64, epoch: u64) {
    let _ = command(device_id, epoch, false);
    let _ = mk_device_release(device_id);
}

pub fn after(device_id: u64, epoch: u64, mmio: &MmioMapOut, ver: MacVersion) {
    release(device_id, epoch, &Regs::new(mmio.user_va), ver);
}

/// Best-effort throughout: a part that will not stop or reset still goes
/// back, with Bus Master off, and the broker's own stop runs before any
/// frame is freed.
pub fn release(device_id: u64, epoch: u64, regs: &Regs, ver: MacVersion) {
    // SAFETY: ChipCmd (0x37) lies inside every mapped window, and the driver
    // owns the part.
    unsafe { regs.w8(REG_CMD, 0) };
    let _ = reset_part(regs, ver);
    unclaim(device_id, epoch);
}
