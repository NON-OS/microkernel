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

//! Which program an installed package starts as, recorded for `run`.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs;
use nonos_libc::mk_getpid;

use super::auth::Verified;
use super::tar::entries;

// Outside every family's tree, where no guest can rewrite what its package
// starts as. One directory per family, so two families' `jq` do not collide.
use crate::linux::file::family::records;

/// Record `usr/bin/<name>` if the package has it, else its first program.
/// True when it is recorded, or the package has no program to record.
pub(super) fn record(name: &str, files: &Verified) -> bool {
    let all = entries(files.files());
    let mut programs = all
        .iter()
        .filter(|e| e.name.starts_with(b"usr/bin/") && e.body.starts_with(b"\x7fELF"))
        .map(|e| e.name.as_slice());
    let own = [b"usr/bin/".as_slice(), name.as_bytes()].concat();
    let chosen = match all.iter().any(|e| e.name == own) {
        true => Some(own.as_slice()),
        false => programs.next(),
    };
    let Some(path) = chosen else { return true };
    let pid = mk_getpid();
    make_records(pid);
    vfs::write_file(pid, &at(name), &[b"/".as_slice(), path].concat()).is_ok()
}

/// The directories the records sit in; ones already there are left alone.
pub(super) fn make_records(pid: u32) {
    for dir in [b"/nonos".as_slice(), b"/nonos/linux", records()] {
        let _ = vfs::mkdir(pid, dir);
    }
}

/// The guest-visible path `name` starts as, if it was installed.
pub fn recorded(name: &str) -> Option<Vec<u8>> {
    vfs::read_file(mk_getpid(), &at(name), 256).ok().filter(|p| p.starts_with(b"/"))
}

/// Forget what `name` starts as, when it is uninstalled. True when there is
/// no record left, whether or not there was one.
pub(super) fn forget(name: &str) -> bool {
    let at = at(name);
    let pid = mk_getpid();
    vfs::unlink(pid, &at).is_ok() || vfs::stat(pid, &at).is_err()
}

fn at(name: &str) -> Vec<u8> {
    [records(), b"/", name.as_bytes()].concat()
}
