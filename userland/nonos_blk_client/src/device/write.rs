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

//! Writes in 512-byte sectors, turned into the disk's own blocks by
//! `span.rs`, which reads a block the write covers only in part before it
//! writes it back whole. Every failure is named on the kernel log before
//! it is returned.

use alloc::vec::Vec;

use super::handle::BlockDevice;
use super::native::Wire;
use super::refused::refused;
use super::span;
use crate::error::BlkError;
use crate::wire::{call, decode_reply, rw_header, HDR_LEN, SECTOR_SIZE, STATUS_LEN};

impl BlockDevice {
    pub fn write(&self, lba: u64, data: &[u8]) -> Result<(), BlkError> {
        if data.is_empty() || !data.len().is_multiple_of(SECTOR_SIZE) {
            refused("write", lba, data.len() / SECTOR_SIZE, &BlkError::Inval);
            return Err(BlkError::Inval);
        }
        span::write(&mut Wire(self), &self.geometry, lba, data)
    }

    /// One request: `chunk` is a whole number of the disk's blocks, no more
    /// than one request may move.
    pub(super) fn write_one(&self, at: u64, chunk: &[u8]) -> Result<(), BlkError> {
        let op = self.driver.ops().write;
        let blocks = chunk.len() / self.geometry.lba_size() as usize;
        let mut body: Vec<u8> = Vec::with_capacity(12 + chunk.len());
        body.extend_from_slice(&rw_header(at, blocks as u32));
        body.extend_from_slice(chunk);
        let mut rx = [0u8; HDR_LEN + STATUS_LEN];
        let (n, id) = call(self.port, self.driver.magic(), op, &body, &mut rx)?;
        decode_reply(&rx, n, self.driver.magic(), op, id).map(|_| ())
    }
}
