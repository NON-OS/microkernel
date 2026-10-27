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

//! A file a shared mapping may be made of because nothing can write it.
//!
//! A shared file mapping is refused (call/mem/map_kind.rs): nothing gives two
//! address spaces one frame. libxkbcommon maps its rules and keymap files
//! with MAP_SHARED and PROT_READ, as do icon and font caches, and GTK's keymap
//! failed without them ("Couldn't read rules file ... Function not
//! implemented"), and the program after it. Read through a descriptor not
//! open to write, a file under /usr is package data the guest cannot change,
//! so a copy of it is what a shared mapping would show. Anywhere else a file
//! may be written while it is mapped (LMDB reads its database this way and
//! writes it with write()), and a copy would go stale, so it stays refused.

use crate::linux::guest::{Fd, Guest, Kind};

const SYSTEM: &[u8] = b"/usr/";

/// Whether `fd` reads a file under /usr and was not opened to write. A path
/// that climbs out with `..` is not taken at its word.
pub fn is_system_file(guest: &Guest, fd: u64) -> bool {
    let reads = |e: &Fd| e.kind == Kind::File && !e.writable && under_usr(&e.path);
    guest.fds.get(fd as usize).is_some_and(reads)
}

fn under_usr(path: &[u8]) -> bool {
    path.starts_with(SYSTEM) && !path.split(|b| *b == b'/').any(|part| part == b"..")
}
