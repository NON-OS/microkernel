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

//! The closure an install of a package needed, kept in the family's records
//! under `.closure/<package>` for `run`, which loads only its packs. Every
//! package of the closure is fetched and placed again by each install, those
//! an earlier install brought too, so the names the install placed are the
//! whole closure.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs;
use nonos_libc::{mk_debug, mk_getpid};

use super::super::Why;
use crate::linux::file::family::records;
use crate::linux::file::pack::{decode_closure, encode_closure, CLOSURE_MAX};
use crate::linux::whole::{ask, whole};

/// Keep `names` as `name`'s closure. One that cannot be kept leaves no
/// record, so its run loads every pack; one that cannot be taken out either
/// would load too few, and fails the install as Partial.
pub(in crate::linux::install) fn keep(name: &str, names: &[String]) -> Result<(), Why> {
    let pid = mk_getpid();
    super::program::make_records(pid);
    let _ = vfs::mkdir(pid, &dir());
    let kept = encode_closure(names).is_some_and(|b| vfs::write_file(pid, &at(name), &b).is_ok());
    if kept {
        return Ok(());
    }
    let line = b"[LINUX] its closure was not kept, so its run loads every pack\n";
    let _ = mk_debug(line.as_ptr(), line.len());
    forget(name).then_some(()).ok_or(Why::PARTIAL)
}

/// The closure `name`'s install kept, or None when it kept none whole.
pub fn closure(name: &str) -> Option<Vec<String>> {
    let raw = vfs::read_file(mk_getpid(), &at(name), ask(CLOSURE_MAX)).ok();
    raw.and_then(|b| whole(b, CLOSURE_MAX)).and_then(|b| decode_closure(&b))
}

/// Forget `name`'s closure. True when there is no record left.
pub(super) fn forget(name: &str) -> bool {
    let (at, pid) = (at(name), mk_getpid());
    vfs::unlink(pid, &at).is_ok() || vfs::stat(pid, &at).is_err()
}

fn dir() -> Vec<u8> {
    [records(), b"/.closure"].concat()
}

fn at(name: &str) -> Vec<u8> {
    [dir().as_slice(), b"/", name.as_bytes()].concat()
}
