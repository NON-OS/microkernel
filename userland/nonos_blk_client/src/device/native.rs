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

//! The driver requests `span.rs` asks for, over the wire. A request that
//! fails is logged here, in the disk's own blocks, which is what the
//! driver was sent and what its own log names.

use super::handle::BlockDevice;
use super::refused::refused_blocks;
use super::span::Native;
use crate::error::BlkError;

pub struct Wire<'a>(pub &'a BlockDevice);

impl Native for Wire<'_> {
    type Error = BlkError;

    fn read_native(&mut self, lba: u64, out: &mut [u8]) -> Result<(), BlkError> {
        self.0.read_one(lba, out).inspect_err(|e| refused_blocks("read", self.0, lba, out.len(), e))
    }

    fn write_native(&mut self, lba: u64, data: &[u8]) -> Result<(), BlkError> {
        self.0
            .write_one(lba, data)
            .inspect_err(|e| refused_blocks("write", self.0, lba, data.len(), e))
    }
}
