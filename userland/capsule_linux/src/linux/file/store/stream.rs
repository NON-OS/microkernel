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

//! A file opened for reading: a handle on the store's own file; a packed
//! file with page digests, each page read when it is first asked for and
//! served only as its digest, then kept for the descriptor; or a version 1
//! packed file's bytes, read whole and checked against its digest once, when
//! it is opened (pack/serve.rs).

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::VfsStream;

use super::bytes::Fail;
use super::pack::read_pages;
use super::pages::assemble;
use crate::linux::start_say::clock;

pub enum Stream {
    Store(VfsStream),
    Packed(Vec<u8>),
    Paged(Paged),
}

/// A version 2 packed file: where it is, its whole digest (the file it was
/// when opened), its size, and the pages read so far, each checked.
pub struct Paged {
    pub at: Vec<u8>,
    pub digest: [u8; 16],
    pub size: u64,
    got: BTreeMap<usize, Vec<u8>>,
}

impl Paged {
    pub fn new(at: Vec<u8>, digest: [u8; 16], size: u64) -> Self {
        Paged { at, digest, size, got: BTreeMap::new() }
    }

    /// Up to `len` bytes from `offset`. The pages not yet held are read in
    /// runs, one window a run, each page checked before it is kept.
    pub fn read_window(&mut self, offset: u64, len: u32) -> Result<Vec<u8>, Fail> {
        let Paged { at, digest, size, got } = self;
        let fetch = |pages| read_pages(at, digest, pages);
        assemble(*size, offset, len, got, fetch, "a packed page came back cut")
    }
}

impl Stream {
    /// Up to `len` bytes from `offset`, as VfsStream reads them.
    pub fn read_window(&mut self, offset: u64, len: u32) -> Result<Vec<u8>, Fail> {
        match self {
            Stream::Store(s) => s.read_window(offset, len).inspect(|b| clock::read(b.len())),
            Stream::Packed(bytes) => {
                let start = usize::try_from(offset).unwrap_or(usize::MAX).min(bytes.len());
                let end = start.saturating_add(len as usize).min(bytes.len());
                Ok(bytes[start..end].to_vec())
            }
            Stream::Paged(paged) => paged.read_window(offset, len),
        }
    }
}
