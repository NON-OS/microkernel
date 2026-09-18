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

//! Reads, one driver request per ceiling-sized piece. Every failure is
//! named on the kernel log before it is returned.

use super::handle::BlockDevice;
use super::refused::refused;
use crate::error::BlkError;
use crate::wire::{call, decode_reply, rw_header, HDR_LEN, MAX_BYTES, SECTOR_SIZE, STATUS_LEN};

impl BlockDevice {
    pub fn read(&self, lba: u64, out: &mut [u8]) -> Result<(), BlkError> {
        if out.is_empty() || !out.len().is_multiple_of(SECTOR_SIZE) {
            refused("read", lba, out.len() / SECTOR_SIZE, &BlkError::Inval);
            return Err(BlkError::Inval);
        }
        for (i, chunk) in out.chunks_mut(MAX_BYTES).enumerate() {
            let at = lba + (i * MAX_BYTES / SECTOR_SIZE) as u64;
            let sectors = chunk.len() / SECTOR_SIZE;
            self.read_one(at, chunk).map_err(|e| {
                refused("read", at, sectors, &e);
                e
            })?;
        }
        Ok(())
    }

    fn read_one(&self, at: u64, chunk: &mut [u8]) -> Result<(), BlkError> {
        let op = self.driver.ops().read;
        let body = rw_header(at, (chunk.len() / SECTOR_SIZE) as u32);
        let mut rx = alloc::vec![0u8; HDR_LEN + STATUS_LEN + chunk.len()];
        let (n, id) = call(self.port, self.driver.magic(), op, &body, &mut rx)?;
        let payload = decode_reply(&rx, n, self.driver.magic(), op, id)?;
        if payload.len() != chunk.len() {
            return Err(BlkError::BadLength);
        }
        chunk.copy_from_slice(payload);
        Ok(())
    }
}
