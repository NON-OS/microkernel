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

//! The live state the server loop holds.

use nonos_libc::{mk_dma_unmap, DmaMapOut};

use super::grants::Grants;
use crate::channel::Channel;
use crate::constants::buffer::{DATA_BYTES, DATA_OFFSET, PRD_OFFSET, PRD_TABLE_BYTES};
use crate::constants::pci::CHANNELS;
use crate::disk::{Disk, UNITS};

pub struct Driver {
    pub grants: Grants,
    pub dma: DmaMapOut,
    pub channels: [Option<Channel>; CHANNELS],
    pub disks: [Option<Disk>; UNITS],
}

impl Driver {
    pub fn disk(&self, unit: u16) -> Option<Disk> {
        self.disks.get(unit as usize).copied().flatten()
    }

    pub fn channel(&self, disk: &Disk) -> Option<Channel> {
        self.channels[disk.channel]
    }

    pub fn disk_count(&self) -> usize {
        self.disks.iter().flatten().count()
    }

    /// The descriptor table's bus address; the grant sits below 4 GiB.
    pub fn prd_dev(&self) -> u32 {
        (self.dma.device_addr + PRD_OFFSET) as u32
    }

    pub fn data_dev(&self) -> u64 {
        self.dma.device_addr + DATA_OFFSET
    }

    pub fn prd_table(&mut self) -> &mut [u8] {
        let at = self.dma.user_va + PRD_OFFSET;
        // SAFETY: the table is the first PRD_TABLE_BYTES of the grant, which
        // stays mapped while the driver lives; `&mut self` keeps it unique.
        unsafe { core::slice::from_raw_parts_mut(at as *mut u8, PRD_TABLE_BYTES) }
    }

    /// The transfer buffer's first `len` bytes, `len` at most DATA_BYTES.
    pub fn data(&mut self, len: usize) -> &mut [u8] {
        let at = self.dma.user_va + DATA_OFFSET;
        // SAFETY: as above, for the buffer after the table's page; the
        // length is cut to the buffer.
        unsafe { core::slice::from_raw_parts_mut(at as *mut u8, len.min(DATA_BYTES)) }
    }

    /// Give back the DMA grant, every port grant and the claim.
    pub fn release(&self) {
        let _ = mk_dma_unmap(self.dma.grant_id);
        self.grants.release();
    }
}
