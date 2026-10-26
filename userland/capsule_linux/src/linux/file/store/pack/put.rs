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

//! A pack put in and taken out through the store (swap.rs says the order).
//! This process's own hold is let go first, so it is never the one in the
//! way, and after, so the next read loads what is there now.

use nonos_app_skeleton::clients::vfs;
use nonos_libc::mk_getpid;

use super::shelf::{Held, Shelf, Unlinked};
use super::swap::{swap, take};
use super::table::forget;

/// The store as vfs answers this process.
struct Store(u32);

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
}

pub fn put(package: &str, pack: &[u8], trailer: &[u8]) -> Result<(), Held> {
    forget();
    let put = swap(&mut Store(mk_getpid()), package, pack, trailer);
    forget();
    put
}

pub fn remove(package: &str) -> Result<(), Held> {
    forget();
    let out = take(&mut Store(mk_getpid()), package);
    forget();
    out
}
