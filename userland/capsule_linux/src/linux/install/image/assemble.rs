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

//! An image read out of its chunks, never whole: its head from the first
//! chunks, then each member from the chunks that hold it. Members lie in
//! image order, so the last chunk read is kept and each chunk is read and
//! held to its digest once, however many members share it. Pure, so the
//! host proofs hold it.

use alloc::vec::Vec;
use core::ops::Range;

use nonos_app_image::{covering, head_len, ChunkList, HEADER_LEN};

/// Chunks as `fetch` hands them back, each already held to its digest,
/// with the last one kept.
pub struct Chunks<'a> {
    fetch: &'a mut dyn FnMut(usize) -> Option<Vec<u8>>,
    last: Option<(usize, Vec<u8>)>,
}

impl<'a> Chunks<'a> {
    pub fn new(fetch: &'a mut dyn FnMut(usize) -> Option<Vec<u8>>) -> Self {
        Chunks { fetch, last: None }
    }

    fn get(&mut self, i: usize) -> Option<Vec<u8>> {
        if let Some((at, bytes)) = &self.last {
            if *at == i {
                return Some(bytes.clone());
            }
        }
        let bytes = (self.fetch)(i)?;
        self.last = Some((i, bytes.clone()));
        Some(bytes)
    }
}

/// Bytes `at` of the image, cut out of the chunks that hold them.
pub fn bytes(list: &ChunkList, at: &Range<u64>, chunks: &mut Chunks) -> Option<Vec<u8>> {
    let held: Option<Vec<Vec<u8>>> = covering(list, at).map(|i| chunks.get(i)).collect();
    let held = held?;
    let parts: Vec<&[u8]> = held.iter().map(Vec::as_slice).collect();
    list.cut(at, &parts)
}

/// The image's head: its header, member table and names.
pub fn head(list: &ChunkList, chunks: &mut Chunks) -> Option<Vec<u8>> {
    let first = bytes(list, &(0..(HEADER_LEN as u64).min(list.total)), chunks)?;
    let len = head_len(&first).ok()? as u64;
    if len > list.total {
        return None;
    }
    bytes(list, &(0..len), chunks)
}
