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

/* A streamed file's writer as bytes and back. A stream cut by a reboot goes
 * on from the last state saved: the blocks that state names are on the disk,
 * and any sealed after it are never pointed at. */

use alloc::vec::Vec;

use super::FileStream;
use crate::fs::blockfs::file_consts::{DATA_BYTES, FANOUT, LEVELS, MAX_FILE_BLOCKS, ROOTS};
use crate::fs::blockfs::tree_shape::locate;

/*
 * Words before the tail: size, tail length, data and pointer block counts,
 * the roots, the pending pointer blocks and how full each is.
 */
const WORDS: usize = 4 + ROOTS + LEVELS * FANOUT + LEVELS;
const STATE_BYTES: usize = WORDS * 8 + DATA_BYTES;

impl FileStream {
    /* This stream as bytes `load` takes back. */
    pub fn save(&self) -> Vec<u8> {
        let t = &self.tree;
        let head = [self.size, self.tail_len as u64, t.data_blocks, t.pointer_blocks];
        let fill = t.fill.iter().map(|&f| f as u64);
        let words = head.iter().chain(&t.root).chain(t.pending.iter().flatten()).copied();
        let mut out = Vec::with_capacity(STATE_BYTES);
        for w in words.chain(fill) {
            out.extend_from_slice(&w.to_le_bytes());
        }
        out.extend_from_slice(&self.tail);
        out
    }

    /* The stream `save` wrote, or `None` for bytes that describe none. */
    pub fn load(bytes: &[u8]) -> Option<FileStream> {
        if bytes.len() != STATE_BYTES {
            return None;
        }
        let word =
            |i: usize| u64::from_le_bytes(bytes[i * 8..i * 8 + 8].try_into().unwrap_or([0; 8]));
        let (size, tail_len, blocks, pointers) = (word(0), word(1), word(2), word(3));
        let whole = blocks.checked_mul(DATA_BYTES as u64)?.checked_add(tail_len)?;
        if tail_len >= DATA_BYTES as u64 || blocks > MAX_FILE_BLOCKS || whole != size {
            return None;
        }
        let mut s = FileStream::new();
        (s.size, s.tail_len) = (size, tail_len as usize);
        (s.tree.data_blocks, s.tree.pointer_blocks) = (blocks, pointers);
        s.tree.root.iter_mut().enumerate().for_each(|(i, r)| *r = word(4 + i));
        for (i, p) in s.tree.pending.iter_mut().flatten().enumerate() {
            *p = word(4 + ROOTS + i);
        }
        for (l, f) in s.tree.fill.iter_mut().enumerate() {
            *f = usize::try_from(word(WORDS - LEVELS + l)).ok().filter(|&f| f < FANOUT)?;
        }
        s.tree.open = blocks.checked_sub(1).and_then(locate).filter(|p| p.depth > 0);
        s.tail.copy_from_slice(&bytes[WORDS * 8..]);
        Some(s)
    }
}
