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

//! The store as vfs answers this process, for a pack's swap (swap.rs).

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::{self, VfsStream};

use super::shelf::{Shelf, Unlinked};

pub(super) struct Store(pub(super) u32);

impl Shelf for Store {
    fn write(&mut self, at: &[u8], bytes: &[u8]) -> bool {
        vfs::write_file(self.0, at, bytes).is_ok()
    }

    fn unlink(&mut self, at: &[u8]) -> Unlinked {
        match vfs::unlink(self.0, at) {
            Ok(()) | Err("not found") => Unlinked::Gone,
            Err("busy") => Unlinked::Busy,
            Err(_) => Unlinked::Failed,
        }
    }

    fn rename(&mut self, from: &[u8], to: &[u8]) -> bool {
        vfs::rename(self.0, from, to).is_ok()
    }

    fn head(&mut self, at: &[u8], len: usize) -> Option<Vec<u8>> {
        let want = u32::try_from(len).ok()?;
        VfsStream::open(self.0, at).ok()?.read_window(0, want).ok().filter(|b| b.len() == len)
    }
}
