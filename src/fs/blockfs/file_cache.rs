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

//! What one file's reads keep for its next read.
//!
//! A large file is read in many calls. Each would otherwise open its index
//! block again, walk the pointer path down again and throw away the run of
//! sectors it fetched ahead. The cache keeps, for the last file read, its
//! tree reader (one pointer block per level) and its run, under an id that
//! names the file and the volume's state: the node and index block, the
//! size, the volume's generation, and the count of writes made to it. A
//! read after any write has another id and starts afresh.
//!
//! What is kept is never trusted as plain data: the run holds sectors still
//! sealed, each opened and checked against its own LBA whenever a block is
//! taken from it, and the pointer blocks were opened the same way.

use super::tree_reader::TreeReader;

/// The file a kept state belongs to, and the volume's state it was read in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FileId {
    pub volume: [u8; 16],
    pub generation: u64,
    /// Writes made to the volume before the read began.
    pub epoch: u64,
    pub node_lba: u64,
    pub index_lba: u64,
    pub size: u64,
}

/// A file's read state: its pointer path and its run of sectors.
pub(crate) struct Held<A> {
    pub reader: TreeReader,
    pub ahead: A,
}

/// The state kept for one file, the last one read.
pub(crate) struct FileCache<A> {
    kept: Option<(FileId, Held<A>)>,
}

impl<A> FileCache<A> {
    pub(crate) const fn new() -> Self {
        FileCache { kept: None }
    }

    /// The state kept for `id`, taken out while it is used. Anything kept
    /// for another id is dropped, as the next `keep` would replace it.
    pub(crate) fn take(&mut self, id: &FileId) -> Option<Held<A>> {
        match self.kept.take() {
            Some((kept, held)) if kept == *id => Some(held),
            _ => None,
        }
    }

    /// Keep `held` for the next read of `id`.
    pub(crate) fn keep(&mut self, id: FileId, held: Held<A>) {
        self.kept = Some((id, held));
    }
}
