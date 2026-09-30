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

//! The running system's vfs as the source of what an install carries, read
//! through the same client every desktop app reads its files with.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs;
use nonos_disk::CarrySource;
use nonos_libc::mk_getpid;

pub struct VfsSource {
    pid: u32,
}

impl VfsSource {
    pub fn new() -> VfsSource {
        VfsSource { pid: mk_getpid() }
    }
}

impl CarrySource for VfsSource {
    fn list(&mut self, prefix: &str) -> Vec<String> {
        vfs::list_paths(self.pid, prefix.as_bytes()).unwrap_or_default()
    }

    /// Sized first, so a file that changed between the size and the read
    /// is not carried half.
    fn read(&mut self, path: &str) -> Option<Vec<u8>> {
        let (size, is_dir) = vfs::stat(self.pid, path.as_bytes()).ok()?;
        let size = u32::try_from(size).ok().filter(|_| !is_dir)?;
        let data = vfs::read_file(self.pid, path.as_bytes(), size).ok()?;
        (data.len() == size as usize).then_some(data)
    }
}
