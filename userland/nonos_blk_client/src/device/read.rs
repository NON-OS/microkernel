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

//! Reads in 512-byte sectors, turned into the disk's own blocks by
//! `span.rs` and sent one driver request per piece. Every failure is named
//! on the kernel log before it is returned.

use super::handle::BlockDevice;
use super::native::Wire;
use super::refused::refused;
use super::span;
use crate::error::BlkError;
use crate::wire::{call, decode_reply, rw_header, HDR_LEN, SECTOR_SIZE, STATUS_LEN};

impl BlockDevice {
    pub fn read(&self, lba: u64, out: &mut [u8]) -> Result<(), BlkError> {
        if out.is_empty() || !out.len().is_multiple_of(SECTOR_SIZE) {
            refused("read", lba, out.len() / SECTOR_SIZE, &BlkError::Inval);
            return Err(BlkError::Inval);
        }
        span::read(&mut Wire(self), &self.geometry, lba, out)
    }

    /// One request: `chunk` is a whole number of the disk's blocks, no more
    /// than one request may move.
    pub(super) fn read_one(&self, at: u64, chunk: &mut [u8]) -> Result<(), BlkError> {
        let op = self.driver.ops().read;
        let blocks = chunk.len() / self.geometry.lba_size() as usize;
        let body = rw_header(at, blocks as u32);
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
