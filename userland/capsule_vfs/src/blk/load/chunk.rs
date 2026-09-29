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

//! One chunk of the entry being staged, and the entry's end.

use alloc::vec;

use super::super::client::read_blocks;
use super::super::store::{finish_entry, sector_span};
use super::super::wire::{MAX_READ_BYTES, SECTOR_SIZE};
use super::types::Load;

impl Load {
    /// Read one chunk of the current entry, finishing it when complete.
    pub(super) fn read_chunk(&mut self) -> Result<(), super::super::error::BlkError> {
        let entry = &self.toc[self.idx];
        let done = self.data.len() as u64;
        if done >= entry.len {
            let entry = self.toc[self.idx].clone();
            let data = core::mem::take(&mut self.data);
            self.staged.push(finish_entry(&entry, data)?);
            self.idx += 1;
            return Ok(());
        }
        let want = core::cmp::min(entry.len - done, MAX_READ_BYTES as u64) as usize;
        let span = sector_span(want);
        let lba = (entry.offset + done) / SECTOR_SIZE as u64;
        let mut scratch = vec![0u8; span];
        read_blocks(lba, &mut scratch)?;
        self.data.extend_from_slice(&scratch[..want]);
        Ok(())
    }
}
