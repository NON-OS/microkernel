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

//! One package's manifest, where it is kept, written and read back.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs;
use nonos_libc::{mk_debug, mk_getpid};

use super::super::manifest::{decode, encode, Manifest};
use crate::linux::file::family::records;

/// The largest manifest read back: a package of some thousands of files.
const MAX: u32 = 1 << 20;

pub(super) fn dir() -> Vec<u8> {
    [records(), b"/.files"].concat()
}

pub(super) fn at(name: &str) -> Vec<u8> {
    [dir().as_slice(), b"/", name.as_bytes()].concat()
}

/// Keep what `name` wrote. False, said on the log, when it cannot be kept:
/// an install an uninstall could not find is not reported installed
/// (`place::unpack` takes it out again).
pub(in super::super) fn write(name: &str, m: &Manifest) -> bool {
    let pid = mk_getpid();
    super::super::record::program::make_records(pid);
    let _ = vfs::mkdir(pid, &dir());
    let kept = vfs::write_file(pid, &at(name), &encode(m)).is_ok();
    if !kept {
        let line = b"[LINUX] what it wrote could not be recorded, so it is taken out again\n";
        let _ = mk_debug(line.as_ptr(), line.len());
    }
    kept
}

/// What `name` wrote, or None when no install of it is recorded.
pub(in super::super) fn read(name: &str) -> Option<Manifest> {
    vfs::read_file(mk_getpid(), &at(name), MAX).ok().map(|b| decode(&b))
}
