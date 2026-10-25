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

use super::super::auth::Verified;
use super::super::market::choose::choose;
use super::super::tar::entries;

// Outside every family's tree, where no guest can rewrite what its package
// starts as. One directory per family, so two families' `jq` do not collide.
use crate::linux::file::family::records;
use crate::linux::whole::{ask, whole};

/// Record the program market::choose picks: the one a market release's
/// policy names, else `usr/bin/<name>`, else the package's first program.
/// True when it is recorded, or the package has no program to record; a
/// policy naming bytes the package does not carry records nothing, and bind
/// refuses the install.
pub(in crate::linux::install) fn record(name: &str, files: &Verified) -> bool {
    let all = entries(files.files());
    let pairs: Vec<(&[u8], &[u8])> =
        all.iter().map(|e| (e.name.as_slice(), e.body.as_slice())).collect();
    let want = super::super::market::hold::wanted();
    let hash = |b: &[u8]| *blake3::hash(b).as_bytes();
    let Some(path) = choose(&pairs, name.as_bytes(), want.as_ref(), hash) else { return true };
    let pid = mk_getpid();
    make_records(pid);
    vfs::write_file(pid, &at(name), &[b"/".as_slice(), path].concat()).is_ok()
}

/// The directories the records sit in; ones already there are left alone.
pub(in crate::linux::install) fn make_records(pid: u32) {
    for dir in [b"/nonos".as_slice(), b"/nonos/linux", records()] {
        let _ = vfs::mkdir(pid, dir);
    }
}

/// The guest-visible path `name` starts as, if it was installed.
pub fn recorded(name: &str) -> Option<Vec<u8>> {
    let raw = vfs::read_file(mk_getpid(), &at(name), ask(256)).ok();
    raw.and_then(|p| whole(p, 256)).filter(|p| p.starts_with(b"/"))
}

/// Forget what `name` starts as, when it is uninstalled. True when there is
/// no record left, whether or not there was one.
pub(in crate::linux::install) fn forget(name: &str) -> bool {
    let at = at(name);
    let pid = mk_getpid();
    vfs::unlink(pid, &at).is_ok() || vfs::stat(pid, &at).is_err()
}

fn at(name: &str) -> Vec<u8> {
    [records(), b"/", name.as_bytes()].concat()
}
