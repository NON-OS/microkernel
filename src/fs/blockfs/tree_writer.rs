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

//! Building a file's index as its data blocks arrive, in order. Each level
//! has at most one pointer block filling; it is written when full or at the
//! end, and its address goes one level up, the top one's into its root slot.
//! Memory stays at LEVELS pointer blocks whatever the file's size.

use super::file_consts::{FANOUT, LEVELS, ROOTS};
use super::tree_shape::{locate, Place};
use super::tree_store::{BlockStore, TreeFault};

pub(crate) struct TreeWriter {
    pub root: [u64; ROOTS],
    /// Data blocks placed so far.
    pub data_blocks: u64,
    /// Pointer blocks written so far.
    pub pointer_blocks: u64,
    /// The pointer block filling at each level, the one above data first.
    pub(super) pending: [[u64; FANOUT]; LEVELS],
    pub(super) fill: [usize; LEVELS],
    pub(super) open: Option<Place>,
}

impl TreeWriter {
    pub(crate) fn new() -> Self {
        TreeWriter {
            root: [0; ROOTS],
            data_blocks: 0,
            pointer_blocks: 0,
            pending: [[0; FANOUT]; LEVELS],
            fill: [0; LEVELS],
            open: None,
        }
    }

    /// Hang the next data block, written at `lba`, in the index.
    pub(crate) fn push<S: BlockStore>(
        &mut self,
        s: &mut S,
        lba: u64,
    ) -> Result<(), TreeFault<S::Error>> {
        let place = locate(self.data_blocks).ok_or(TreeFault::TooLarge)?;
        self.data_blocks += 1;
        if place.depth == 0 {
            self.root[place.slot] = lba;
            return Ok(());
        }
        self.open = Some(place);
        let mut carry = lba;
        for level in 0..place.depth {
            self.pending[level][self.fill[level]] = carry;
            self.fill[level] += 1;
            if self.fill[level] < FANOUT {
                return Ok(());
            }
            carry = self.write_level(s, level)?;
        }
        self.root[place.slot] = carry;
        Ok(())
    }
}
