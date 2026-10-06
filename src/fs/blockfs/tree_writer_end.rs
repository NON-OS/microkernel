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

//! Closing a file's index: the pointer blocks still filling are written.

use super::file_consts::FANOUT;
use super::tree_ptrs::encode;
use super::tree_store::{BlockStore, TreeFault};
use super::tree_writer::TreeWriter;

impl TreeWriter {
    /// Write out every pointer block still filling. The index is complete after.
    pub(crate) fn finish<S: BlockStore>(&mut self, s: &mut S) -> Result<(), TreeFault<S::Error>> {
        let Some(place) = self.open.take() else {
            return Ok(());
        };
        for level in 0..place.depth {
            if self.fill[level] == 0 {
                continue;
            }
            let up = self.write_level(s, level)?;
            if level + 1 == place.depth {
                self.root[place.slot] = up;
            } else {
                self.pending[level + 1][self.fill[level + 1]] = up;
                self.fill[level + 1] += 1;
            }
        }
        Ok(())
    }

    pub(super) fn write_level<S: BlockStore>(
        &mut self,
        s: &mut S,
        level: usize,
    ) -> Result<u64, TreeFault<S::Error>> {
        let lba = s.alloc().map_err(TreeFault::Store)?;
        s.put(lba, &encode(&self.pending[level])).map_err(TreeFault::Store)?;
        self.pending[level] = [0u64; FANOUT];
        self.fill[level] = 0;
        self.pointer_blocks += 1;
        Ok(lba)
    }
}
