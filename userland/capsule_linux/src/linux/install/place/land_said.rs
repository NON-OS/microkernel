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
//! Putting a package's files into the store, under the Linux root.
//! What landing a pack says in the log: how many names the store holds
//! before and after, so a pack's one name is measured, and why one failed.

use alloc::format;

use nonos_app_skeleton::clients::vfs;
use nonos_libc::{mk_debug, mk_getpid};
use nonos_linux_pack::Refused;

use super::super::Why;

/// The names the store holds, all owners together; 0 when it will not say.
pub(super) fn names() -> u32 {
    vfs::usage(mk_getpid()).map_or(0, |(files, _, _)| files)
}

pub(super) fn landed(name: &str, files: usize, bytes: usize, before: u32) {
    let line = format!(
        "[LINUX] {name}: {files} file(s) in one pack of {bytes} bytes; store names {before} -> {}\n",
        names()
    );
    let _ = mk_debug(line.as_ptr(), line.len());
}

pub(super) fn refused(name: &str, why: Refused) -> Why {
    match why {
        Refused::Memory => {
            say(name, "too little free memory to pack it");
            Why::UNPACK_MEMORY
        }
        _ => {
            say(name, "its files do not make one pack: past 64 MiB, or a name a pack cannot hold");
            Why::PACKAGE
        }
    }
}

pub(super) fn say(name: &str, why: &str) {
    let line = format!("[LINUX] {name}: {why}; nothing of it is written\n");
    let _ = mk_debug(line.as_ptr(), line.len());
}
