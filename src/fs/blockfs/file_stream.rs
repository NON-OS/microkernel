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

//! Writing a file in pieces of any size, so a file far larger than memory
//! never has to be held whole: bytes are sealed into blocks as they fill.

use super::file_consts::{DATA_BYTES, MAX_FILE_BYTES};
use super::tree_store::Block;
use super::tree_writer::TreeWriter;
use super::{BlockFsError, BlockFsMount};
use crate::fs::cryptoblock::PLAIN_BLOCK_BYTES;

pub struct FileStream {
    pub(super) tree: TreeWriter,
    pub(super) size: u64,
    pub(super) tail: Block,
    pub(super) tail_len: usize,
}

impl FileStream {
    pub fn new() -> Self {
        FileStream { tree: TreeWriter::new(), size: 0, tail: [0u8; PLAIN_BLOCK_BYTES], tail_len: 0 }
    }

    /// Bytes taken so far.
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Take the next `bytes`. Refused whole if they would pass the largest file.
    pub fn append(
        &mut self,
        key: &[u8; 32],
        mount: &mut BlockFsMount,
        mut bytes: &[u8],
    ) -> Result<(), BlockFsError> {
        let after = self.size.checked_add(bytes.len() as u64).ok_or(BlockFsError::OutOfSpace)?;
        if after > MAX_FILE_BYTES {
            return Err(BlockFsError::OutOfSpace);
        }
        while !bytes.is_empty() {
            let take = (DATA_BYTES - self.tail_len).min(bytes.len());
            self.tail[self.tail_len..self.tail_len + take].copy_from_slice(&bytes[..take]);
            self.tail_len += take;
            bytes = &bytes[take..];
            if self.tail_len == DATA_BYTES {
                self.seal_tail(key, mount)?;
            }
        }
        self.size = after;
        Ok(())
    }
}
