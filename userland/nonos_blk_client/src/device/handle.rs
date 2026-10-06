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

//! The handle: which driver, which instance of it, on which port, how big,
//! and the blocks its disk is addressed in.

use nonos_libc::mk_service_lookup;

use super::nvme;
use super::span::Geometry;
use crate::driver::Driver;
use crate::error::BlkError;
use crate::wire::{call, decode_reply, HDR_LEN, STATUS_LEN};

#[derive(Clone, Copy, Debug)]
pub struct BlockDevice {
    pub driver: Driver,
    /// Which of the driver's service names answered: 0 for `driver.nvme0`.
    pub instance: u8,
    pub port: u32,
    /// The size in 512-byte sectors, whatever the disk's own block size.
    pub sectors: u64,
    pub geometry: Geometry,
}

impl BlockDevice {
    /// Look instance `instance` of the driver's service up and ask it how
    /// big its disk is and what blocks it is addressed in. `None` when no
    /// such service is registered on this boot; an error when it is and
    /// did not answer.
    pub fn open(driver: Driver, instance: u8) -> Result<Option<BlockDevice>, BlkError> {
        let Some(name) = driver.service(instance) else {
            return Ok(None);
        };
        let (mut port, mut pid) = (0u32, 0u32);
        let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
        if rc < 0 || port == 0 {
            return Ok(None);
        }
        let geometry = match driver {
            Driver::Nvme => nvme::geometry(port)?,
            Driver::Ahci | Driver::VirtioBlk => Geometry::SECTORS,
        };
        let mut dev = BlockDevice { driver, instance, port, sectors: 0, geometry };
        dev.sectors = geometry.sectors(dev.capacity_blocks()?);
        Ok(Some(dev))
    }

    /// The capacity as the driver gives it, in the disk's own blocks.
    pub fn capacity_blocks(&self) -> Result<u64, BlkError> {
        let ops = self.driver.ops();
        let mut rx = [0u8; HDR_LEN + STATUS_LEN + 8];
        let (n, id) = call(self.port, self.driver.magic(), ops.capacity, &[], &mut rx)?;
        let body = decode_reply(&rx, n, self.driver.magic(), ops.capacity, id)?;
        if body.len() < 8 {
            return Err(BlkError::BadLength);
        }
        let mut w = [0u8; 8];
        w.copy_from_slice(&body[..8]);
        Ok(u64::from_le_bytes(w))
    }

    /// Saturated: the size is the driver's word, and a size shown as a few
    /// bytes because it wrapped would misdescribe the disk.
    pub fn bytes(&self) -> u64 {
        self.sectors.saturating_mul(crate::wire::SECTOR_SIZE as u64)
    }
}
