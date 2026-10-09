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

//! Each installed package's manifest (`manifest`), kept in the family's
//! records under `.files/<package>`. A package name never starts with a dot
//! (the kernel's `package_arg` refuses one), so no package's program record
//! can be mistaken for this directory. Outside every family's tree, where no
//! guest can rewrite what an uninstall would take out.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs;
use nonos_libc::{mk_debug, mk_getpid};

use super::manifest::{decode, encode, Manifest};
use crate::linux::file::family::records;

/// The largest manifest read back: a package of some thousands of files.
const MAX: u32 = 1 << 20;

fn dir() -> Vec<u8> {
    [records(), b"/.files"].concat()
}

fn at(name: &str) -> Vec<u8> {
    [dir().as_slice(), b"/", name.as_bytes()].concat()
}

/// Keep what `name` wrote. False, said on the log, when it cannot be kept:
/// an install an uninstall could not find is not reported installed
/// (`place::unpack` takes it out again).
pub(super) fn write(name: &str, m: &Manifest) -> bool {
    let pid = mk_getpid();
    super::program::make_records(pid);
    let _ = vfs::mkdir(pid, &dir());
    let kept = vfs::write_file(pid, &at(name), &encode(m)).is_ok();
    if !kept {
        let line = b"[LINUX] what it wrote could not be recorded, so it is taken out again\n";
        let _ = mk_debug(line.as_ptr(), line.len());
    }
    kept
}

/// What `name` wrote, or None when no install of it is recorded.
pub(super) fn read(name: &str) -> Option<Manifest> {
    vfs::read_file(mk_getpid(), &at(name), MAX).ok().map(|b| decode(&b))
}

/// Every other package's manifest, so what two packages share stays.
pub(super) fn others(name: &str) -> Vec<Manifest> {
    let listed = vfs::list_paths(mk_getpid(), &dir()).unwrap_or_default();
    let names = listed.iter().map(|p| p.rsplit('/').next().unwrap_or(p)).map(String::from);
    names.filter(|n| !n.is_empty() && n != name).filter_map(|n| read(&n)).collect()
}

/// Keep only what is still to take out, or drop the record once nothing is.
pub(super) fn settle(name: &str, left: &Manifest) -> bool {
    let pid = mk_getpid();
    if left.files.is_empty() && left.links.is_empty() {
        return vfs::unlink(pid, &at(name)).is_ok();
    }
    vfs::write_file(pid, &at(name), &encode(left)).is_ok()
}
