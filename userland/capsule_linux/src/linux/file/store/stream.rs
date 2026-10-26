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

//! A file opened for reading: a handle on the store's own file, or a
//! packed file's bytes, read whole and checked against its digest once,
//! when it is opened (pack/serve.rs).

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::VfsStream;

use super::bytes::Fail;

pub enum Stream {
    Store(VfsStream),
    Packed(Vec<u8>),
}

impl Stream {
    /// Up to `len` bytes from `offset`, as VfsStream reads them.
    pub fn read_window(&mut self, offset: u64, len: u32) -> Result<Vec<u8>, Fail> {
        match self {
            Stream::Store(s) => s.read_window(offset, len),
            Stream::Packed(bytes) => {
                let start = usize::try_from(offset).unwrap_or(usize::MAX).min(bytes.len());
                let end = start.saturating_add(len as usize).min(bytes.len());
                Ok(bytes[start..end].to_vec())
            }
        }
    }
}
