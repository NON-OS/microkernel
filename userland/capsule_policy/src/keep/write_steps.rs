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

//! The settings record's write, as the calls it makes to vfs.
//!
//! vfs serves requests in order and acts on every one it got, so a call that
//! timed out here is still done there, before anything sent after it. A
//! write retried after a timeout therefore runs again over the effects of
//! the first: the directories exist, the file is unlinked and written anew,
//! and the record kept again with the latest bytes. Each call is harmless
//! done twice, so a late one can never leave an older record kept. No crate
//! paths here, so the host proofs mount this file and drive it.

/// The vfs calls the write makes; `Err` is any failure, a late answer too.
pub trait Vfs {
    fn mkdir(&mut self, path: &[u8]) -> Result<(), ()>;
    fn unlink(&mut self, path: &[u8]) -> Result<(), ()>;
    fn write_file(&mut self, path: &[u8], data: &[u8]) -> Result<(), ()>;
    fn persist(&mut self, path: &[u8]) -> Result<(), ()>;
}

/// Write `bytes` to `path` and keep it, after making `dirs` and unlinking
/// the copy an earlier boot kept, which belongs to nobody. A directory that
/// exists and a file already gone are not failures; the write and the keep
/// are, and the caller tries the whole write again later.
pub fn write<V: Vfs>(vfs: &mut V, dirs: &[&[u8]], path: &[u8], bytes: &[u8]) -> Result<(), ()> {
    for dir in dirs {
        let _ = vfs.mkdir(dir);
    }
    let _ = vfs.unlink(path);
    vfs.write_file(path, bytes)?;
    vfs.persist(path)
}

/// The change count the disk holds after a write of `attempt`'s snapshot:
/// that count once the write is kept, else still `written`, so a failed or
/// late write is tried again rather than taken as done.
pub fn written_after(written: u32, attempt: u32, kept: bool) -> u32 {
    if kept {
        attempt
    } else {
        written
    }
}
