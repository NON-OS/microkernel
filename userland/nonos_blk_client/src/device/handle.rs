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

//! The handle: which driver, on which port, and how big.

use nonos_libc::mk_service_lookup;

use crate::driver::Driver;
use crate::error::BlkError;
use crate::wire::{call, decode_reply, HDR_LEN, STATUS_LEN};

#[derive(Clone, Copy, Debug)]
pub struct BlockDevice {
    pub driver: Driver,
    pub port: u32,
    pub sectors: u64,
}

impl BlockDevice {
    /// Look the driver's service up and ask it how big its disk is. `None`
    /// when no such driver is registered on this boot; an error when it is
    /// and did not answer.
    pub fn open(driver: Driver) -> Result<Option<BlockDevice>, BlkError> {
        let name = driver.service();
        let (mut port, mut pid) = (0u32, 0u32);
        let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
        if rc < 0 || port == 0 {
            return Ok(None);
        }
        let mut dev = BlockDevice { driver, port, sectors: 0 };
        dev.sectors = dev.capacity()?;
        Ok(Some(dev))
    }

    pub fn capacity(&self) -> Result<u64, BlkError> {
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

    pub fn bytes(&self) -> u64 {
        self.sectors * crate::wire::SECTOR_SIZE as u64
    }
}
