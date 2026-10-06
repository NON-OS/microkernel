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
use super::super::error::BlkError;
use super::super::store::{finish_entry, sector_span, stream_entry};
use super::super::wire::{MAX_READ_BYTES, SECTOR_SIZE};
use super::types::Load;

impl Load {
    /// Read one chunk of the current entry, finishing it when complete.
    pub(super) fn read_chunk(&mut self) -> Result<(), BlkError> {
        let entry = &self.toc[self.idx];
        let done = self.data.len() as u64;
        if done == 0 && nonos_disk_map::streamed(&entry.name) {
            self.staged.push(stream_entry(entry));
            self.idx += 1;
            return Ok(());
        }
        if done == 0 && entry.len > 0 {
            /*
             * Sized once for the whole entry. Grown by doubling, each step
             * held the old buffer and one twice its size at once, so an entry
             * over 32 MiB wanted a free 64 MiB block beside its 32 MiB one,
             * and every finished entry kept up to twice its length.
             */
            let len = usize::try_from(entry.len).map_err(|_| BlkError::NoMemory)?;
            self.data.try_reserve_exact(len).map_err(|_| BlkError::NoMemory)?;
        }
        if done >= entry.len {
            let entry = self.toc[self.idx].clone();
            let data = core::mem::take(&mut self.data);
            /*
             * A digest that disagrees is that payload damaged, not the
             * store: it is left out and counted, and the walk goes on.
             */
            match finish_entry(&entry, data) {
                Ok(staged) => self.staged.push(staged),
                Err(_) => self.refused += 1,
            }
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
